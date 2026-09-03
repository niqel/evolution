use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use std::collections::HashMap;

use crate::data::ast::expressions::{BinaryOperator, UnaryOperator};
use crate::data::compiled::boundary::{CompiledEnumValueShape, CompiledValueShape};
use crate::data::compiled::equality::{
    CompositeEqualityPlan, EnumEqualityPayloadPlan, EqualityRule,
};
use crate::data::compiled::identities::{
    CompiledValueShapeId, ConstantId, ExternalSymbolId, FieldIndex, InstructionIndex, LocalSlot,
    NumericKind, ParameterSlot, VariantDiscriminant,
};
use crate::data::compiled::instructions::Instruction;
use crate::data::compiled::program::{CompiledFunction, CompiledProgram};
use crate::data::compiled::source_map::SourceMap;
use crate::data::compiled::storage::{Constant, DynamicConstant, ExternalSymbol};
use crate::data::lexical::SourceSpan;
use crate::data::semantic::SignatureSymbol;
use crate::data::semantic::expressions::{
    SemanticArgument, SemanticCallTarget, SemanticEnumPayload, SemanticExpression,
    SemanticExpressionKind, SemanticLiteral, SemanticStatement, SemanticVariantExtraction,
};
use crate::data::semantic::ids::FunctionId;
use crate::data::semantic::structure::{
    NativeType, SemanticFunction, SemanticParameter, SemanticProgram, SemanticSignature,
    SemanticSignatureParameter, SemanticType, SemanticVariant,
};

pub type Lower = fn(&SemanticProgram) -> CompiledProgram;

pub fn lower_program(semantic_program: &SemanticProgram) -> CompiledProgram {
    let mut compiler = BytecodeCompiler::new(semantic_program);
    compiler.compile()
}

pub const LOWER_PROGRAM: Lower = lower_program;

// --- Private Working State ---

struct BytecodeCompiler<'a> {
    semantic_program: &'a SemanticProgram,
    constants: Vec<Constant>,
    external_symbols: Vec<ExternalSymbol>,
    external_symbol_map: HashMap<(String, String, usize, usize), usize>,
    value_shapes: Vec<CompiledValueShape>,
    shape_cache: HashMap<usize, usize>,
}

impl<'a> BytecodeCompiler<'a> {
    fn new(semantic_program: &'a SemanticProgram) -> Self {
        Self {
            semantic_program,
            constants: Vec::new(),
            external_symbols: Vec::new(),
            external_symbol_map: HashMap::new(),
            value_shapes: Vec::new(),
            shape_cache: HashMap::new(),
        }
    }

    fn compile(mut self) -> CompiledProgram {
        let entry_point_idx = self.semantic_program.entry_function.0;

        // 1. Build entry parameter shapes
        let mut entry_parameter_shapes = Vec::new();
        if let Some(entry_fn) = self.semantic_program.functions.get(entry_point_idx) {
            for param in &entry_fn.parameters {
                if let SemanticParameter::Value(bid) = param {
                    let tid = entry_fn.bindings[bid.0].type_id.0;
                    let shape_idx = self.get_or_create_shape(tid);
                    entry_parameter_shapes.push(CompiledValueShapeId(shape_idx));
                }
            }
        }

        // 2. Emit all functions
        let mut compiled_functions = Vec::new();
        let mut source_map_functions = Vec::new();

        for func_idx in 0..self.semantic_program.functions.len() {
            let func = &self.semantic_program.functions[func_idx];
            let (compiled_func, spans) = FunctionEmitter::new(&mut self, func).emit_function();
            compiled_functions.push(compiled_func);
            source_map_functions.push(spans);
        }

        CompiledProgram {
            functions: compiled_functions,
            entry_point: FunctionId(entry_point_idx),
            entry_parameter_shapes,
            constants: self.constants,
            external_symbols: self.external_symbols,
            value_shapes: self.value_shapes,
            source_map: SourceMap {
                functions: source_map_functions,
            },
        }
    }

    fn get_or_create_shape(&mut self, type_id: usize) -> usize {
        if let Some(&shape_idx) = self.shape_cache.get(&type_id) {
            return shape_idx;
        }

        let shape_idx = self.value_shapes.len();
        self.shape_cache.insert(type_id, shape_idx);
        self.value_shapes.push(CompiledValueShape::Boolean);

        let shape = match &self.semantic_program.types[type_id] {
            SemanticType::Native(nt) => match nt {
                NativeType::Bool => CompiledValueShape::Boolean,
                NativeType::Int8 => CompiledValueShape::Int8,
                NativeType::Int16 => CompiledValueShape::Int16,
                NativeType::Int | NativeType::Int32 => CompiledValueShape::Int32,
                NativeType::Int64 => CompiledValueShape::Int64,
                NativeType::Int128 => CompiledValueShape::Int128,
                NativeType::Uint8 => CompiledValueShape::Uint8,
                NativeType::Uint16 => CompiledValueShape::Uint16,
                NativeType::Uint32 => CompiledValueShape::Uint32,
                NativeType::Uint64 => CompiledValueShape::Uint64,
                NativeType::Uint128 => CompiledValueShape::Uint128,
                NativeType::Float32 => CompiledValueShape::Float32,
                NativeType::Float | NativeType::Float64 => CompiledValueShape::Float64,
                NativeType::String => CompiledValueShape::String,
                NativeType::Dynamic => CompiledValueShape::Dynamic,
            },
            SemanticType::Struct { fields } => {
                let mut field_shapes = Vec::with_capacity(fields.len());
                for f in fields {
                    let f_shape_idx = self.get_or_create_shape(f.type_id.0);
                    field_shapes.push(CompiledValueShapeId(f_shape_idx));
                }
                CompiledValueShape::Struct {
                    fields: field_shapes,
                }
            }
            SemanticType::Enum { variants } => {
                let mut variant_shapes = Vec::with_capacity(variants.len());
                for v in variants {
                    let v_shape = match v {
                        SemanticVariant::Simple => CompiledEnumValueShape::Simple,
                        SemanticVariant::Associated { type_id } => {
                            let p_shape_idx = self.get_or_create_shape(type_id.0);
                            CompiledEnumValueShape::Associated(CompiledValueShapeId(p_shape_idx))
                        }
                        SemanticVariant::Structured { fields } => {
                            let mut vf_shapes = Vec::with_capacity(fields.len());
                            for f in fields {
                                let f_shape_idx = self.get_or_create_shape(f.type_id.0);
                                vf_shapes.push(CompiledValueShapeId(f_shape_idx));
                            }
                            CompiledEnumValueShape::Structured { fields: vf_shapes }
                        }
                    };
                    variant_shapes.push(v_shape);
                }
                CompiledValueShape::Enum {
                    variants: variant_shapes,
                }
            }
        };

        self.value_shapes[shape_idx] = shape;
        shape_idx
    }

    fn get_or_create_external_symbol(&mut self, sig: &SemanticSignature) -> usize {
        let param_count = sig
            .parameters
            .iter()
            .filter(|p| matches!(p, SemanticSignatureParameter::Value(_)))
            .count();
        let result_shape_idx = self.get_or_create_shape(sig.result_type.0);

        let key = (
            sig.symbol.module.clone(),
            sig.symbol.name.clone(),
            param_count,
            result_shape_idx,
        );

        if let Some(&esid_idx) = self.external_symbol_map.get(&key) {
            esid_idx
        } else {
            let esid_idx = self.external_symbols.len();
            self.external_symbols.push(ExternalSymbol {
                symbol: SignatureSymbol {
                    module: sig.symbol.module.clone(),
                    name: sig.symbol.name.clone(),
                },
                parameter_count: param_count,
                result_shape: CompiledValueShapeId(result_shape_idx),
            });
            self.external_symbol_map.insert(key, esid_idx);
            esid_idx
        }
    }

    fn build_equality_rule(&self, type_id: usize) -> EqualityRule {
        match &self.semantic_program.types[type_id] {
            SemanticType::Native(nt) => match nt {
                NativeType::Bool => EqualityRule::Boolean,
                NativeType::String => EqualityRule::String,
                NativeType::Int | NativeType::Int32 => EqualityRule::Numeric(NumericKind::Int32),
                NativeType::Int8 => EqualityRule::Numeric(NumericKind::Int8),
                NativeType::Int16 => EqualityRule::Numeric(NumericKind::Int16),
                NativeType::Int64 => EqualityRule::Numeric(NumericKind::Int64),
                NativeType::Int128 => EqualityRule::Numeric(NumericKind::Int128),
                NativeType::Uint8 => EqualityRule::Numeric(NumericKind::Uint8),
                NativeType::Uint16 => EqualityRule::Numeric(NumericKind::Uint16),
                NativeType::Uint32 => EqualityRule::Numeric(NumericKind::Uint32),
                NativeType::Uint64 => EqualityRule::Numeric(NumericKind::Uint64),
                NativeType::Uint128 => EqualityRule::Numeric(NumericKind::Uint128),
                NativeType::Float | NativeType::Float64 => {
                    EqualityRule::Numeric(NumericKind::Float64)
                }
                NativeType::Float32 => EqualityRule::Numeric(NumericKind::Float32),
                NativeType::Dynamic => panic!("dynamic equality not supported"),
            },
            SemanticType::Struct { .. } | SemanticType::Enum { .. } => {
                EqualityRule::Composite(self.build_composite_equality_plan(type_id))
            }
        }
    }

    fn build_composite_equality_plan(&self, type_id: usize) -> CompositeEqualityPlan {
        match &self.semantic_program.types[type_id] {
            SemanticType::Struct { fields } => {
                let field_rules = fields
                    .iter()
                    .map(|f| self.build_equality_rule(f.type_id.0))
                    .collect();
                CompositeEqualityPlan::Struct {
                    fields: field_rules,
                }
            }
            SemanticType::Enum { variants } => {
                let var_plans = variants
                    .iter()
                    .map(|v| match v {
                        SemanticVariant::Simple => EnumEqualityPayloadPlan::Simple,
                        SemanticVariant::Associated { type_id } => {
                            EnumEqualityPayloadPlan::Associated(self.build_equality_rule(type_id.0))
                        }
                        SemanticVariant::Structured { fields } => {
                            let f_rules = fields
                                .iter()
                                .map(|f| self.build_equality_rule(f.type_id.0))
                                .collect();
                            EnumEqualityPayloadPlan::Structured { fields: f_rules }
                        }
                    })
                    .collect();
                CompositeEqualityPlan::Enum {
                    variants: var_plans,
                }
            }
            _ => panic!("expected struct or enum type"),
        }
    }
}

// --- Function Emitter ---

#[derive(Clone, Copy)]
enum Slot {
    Parameter(usize),
    Local(usize),
}

struct FunctionEmitter<'a, 'c> {
    compiler: &'c mut BytecodeCompiler<'a>,
    function: &'a SemanticFunction,

    instructions: Vec<Instruction>,
    spans: Vec<SourceSpan>,

    binding_to_slot: HashMap<usize, Slot>,
    parameter_count: usize,
    local_count: usize,
}

impl<'a, 'c> FunctionEmitter<'a, 'c> {
    fn new(compiler: &'c mut BytecodeCompiler<'a>, function: &'a SemanticFunction) -> Self {
        let mut binding_to_slot = HashMap::new();
        let mut param_index = 0;

        for param in &function.parameters {
            match param {
                SemanticParameter::Value(bid) => {
                    binding_to_slot.insert(bid.0, Slot::Parameter(param_index));
                    param_index += 1;
                }
                SemanticParameter::SignatureDependency(_) => {
                    // Erased from physical slots
                }
            }
        }

        let mut local_index = 0;
        for bid in 0..function.bindings.len() {
            if !binding_to_slot.contains_key(&bid) {
                binding_to_slot.insert(bid, Slot::Local(local_index));
                local_index += 1;
            }
        }

        Self {
            compiler,
            function,
            instructions: Vec::new(),
            spans: Vec::new(),
            binding_to_slot,
            parameter_count: param_index,
            local_count: local_index,
        }
    }

    fn emit(&mut self, instruction: Instruction, span: SourceSpan) {
        self.instructions.push(instruction);
        self.spans.push(span);
    }

    fn emit_placeholder_jump(&mut self, span: SourceSpan) -> usize {
        let pos = self.instructions.len();
        self.emit(Instruction::Jump(InstructionIndex(0)), span);
        pos
    }

    fn emit_placeholder_jump_if_false(&mut self, span: SourceSpan) -> usize {
        let pos = self.instructions.len();
        self.emit(Instruction::JumpIfFalse(InstructionIndex(0)), span);
        pos
    }

    fn patch_jump(&mut self, pos: usize, target: usize) {
        match &mut self.instructions[pos] {
            Instruction::Jump(t) => *t = InstructionIndex(target),
            Instruction::JumpIfFalse(t) => *t = InstructionIndex(target),
            _ => panic!("cannot patch non-jump instruction"),
        }
    }

    fn emit_load_constant(&mut self, constant: Constant, span: SourceSpan) {
        let cid = ConstantId(self.compiler.constants.len());
        self.compiler.constants.push(constant);
        self.emit(Instruction::LoadConstant(cid), span);
    }

    fn adapt_to_expected_type(&mut self, actual_tid: usize, expected_tid: usize, span: SourceSpan) {
        if actual_tid != expected_tid {
            let actual_type = &self.compiler.semantic_program.types[actual_tid];
            let expected_type = &self.compiler.semantic_program.types[expected_tid];
            if matches!(expected_type, SemanticType::Native(NativeType::Dynamic)) {
                if let Some(num_kind) = to_numeric_kind(actual_type) {
                    self.emit(Instruction::LiftDynamic(num_kind), span);
                }
            }
        }
    }

    fn emit_function(mut self) -> (CompiledFunction, Vec<SourceSpan>) {
        for stmt in &self.function.body.statements {
            match stmt {
                SemanticStatement::Bind { binding, value } => {
                    let val_tid = value.type_id.0;
                    let bind_tid = self.function.bindings[binding.0].type_id.0;
                    self.lower_expression(value);
                    self.adapt_to_expected_type(val_tid, bind_tid, value.span);
                    let slot_idx = match self.binding_to_slot[&binding.0] {
                        Slot::Local(l) => l,
                        Slot::Parameter(_) => panic!("let binding cannot be parameter slot"),
                    };
                    self.emit(Instruction::StoreLocal(LocalSlot(slot_idx)), value.span);
                }
                SemanticStatement::Operation(expr) => {
                    self.lower_expression(expr);
                    self.emit(Instruction::Discard, expr.span);
                }
            }
        }

        self.lower_expression(&self.function.body.result);
        self.adapt_to_expected_type(
            self.function.body.result.type_id.0,
            self.function.result_type.0,
            self.function.body.result.span,
        );
        self.emit(Instruction::Return, self.function.body.result.span);

        let fn_param_counts: Vec<usize> = self
            .compiler
            .semantic_program
            .functions
            .iter()
            .map(|f| {
                f.parameters
                    .iter()
                    .filter(|p| matches!(p, SemanticParameter::Value(_)))
                    .count()
            })
            .collect();

        let ext_param_counts: Vec<usize> = self
            .compiler
            .external_symbols
            .iter()
            .map(|e| e.parameter_count)
            .collect();

        let max_operand_depth =
            compute_max_operand_depth(&self.instructions, &fn_param_counts, &ext_param_counts);

        let compiled_function = CompiledFunction {
            parameter_count: self.parameter_count,
            local_count: self.local_count,
            max_operand_depth,
            instructions: self.instructions,
        };

        (compiled_function, self.spans)
    }

    fn lower_expression(&mut self, expr: &SemanticExpression) {
        let span = expr.span;
        let expr_tid = expr.type_id.0;
        let sem_type = &self.compiler.semantic_program.types[expr_tid];

        match &expr.kind {
            SemanticExpressionKind::Literal(lit) => {
                let constant = lower_literal(lit, sem_type);
                self.emit_load_constant(constant, span);
            }
            SemanticExpressionKind::Binding(bid) => match self.binding_to_slot[&bid.0] {
                Slot::Parameter(p) => self.emit(Instruction::LoadParameter(ParameterSlot(p)), span),
                Slot::Local(l) => self.emit(Instruction::LoadLocal(LocalSlot(l)), span),
            },
            SemanticExpressionKind::Unary { operator, operand } => match operator {
                UnaryOperator::Not => {
                    self.lower_expression(operand);
                    self.emit(Instruction::NotBoolean, span);
                }
                UnaryOperator::Negate => {
                    if let SemanticExpressionKind::Literal(lit) = &operand.kind {
                        if let SemanticLiteral::Integer(s) = lit {
                            let constant = match sem_type {
                                SemanticType::Native(nt) => match nt {
                                    NativeType::Int8 => {
                                        let val: i8 = if s == "128" {
                                            -128
                                        } else {
                                            -s.parse::<i8>().unwrap_or(0)
                                        };
                                        Some(Constant::Int8(val))
                                    }
                                    NativeType::Int16 => {
                                        let val: i16 = if s == "32768" {
                                            -32768
                                        } else {
                                            -s.parse::<i16>().unwrap_or(0)
                                        };
                                        Some(Constant::Int16(val))
                                    }
                                    NativeType::Int | NativeType::Int32 => {
                                        let val: i32 = if s == "2147483648" {
                                            -2147483648
                                        } else {
                                            -s.parse::<i32>().unwrap_or(0)
                                        };
                                        Some(Constant::Int32(val))
                                    }
                                    NativeType::Int64 => {
                                        let val: i64 = if s == "9223372036854775808" {
                                            -9223372036854775808
                                        } else {
                                            -s.parse::<i64>().unwrap_or(0)
                                        };
                                        Some(Constant::Int64(val))
                                    }
                                    NativeType::Int128 => {
                                        let val: i128 =
                                            if s == "170141183460469231731687303715884105728" {
                                                -170141183460469231731687303715884105728
                                            } else {
                                                -s.parse::<i128>().unwrap_or(0)
                                            };
                                        Some(Constant::Int128(val))
                                    }
                                    NativeType::Dynamic => {
                                        let magnitude = decimal_string_to_big_endian_magnitude(s);
                                        let is_zero = magnitude.is_empty();
                                        Some(Constant::Dynamic(DynamicConstant::Integer {
                                            negative: !is_zero,
                                            magnitude,
                                        }))
                                    }
                                    _ => None,
                                },
                                _ => None,
                            };

                            if let Some(c) = constant {
                                self.emit_load_constant(c, span);
                                return;
                            }
                        }
                    }

                    self.lower_expression(operand);
                    if matches!(sem_type, SemanticType::Native(NativeType::Dynamic)) {
                        let op_type = &self.compiler.semantic_program.types[operand.type_id.0];
                        if let Some(num_kind) = to_numeric_kind(op_type) {
                            self.emit(Instruction::LiftDynamic(num_kind), operand.span);
                        }
                        self.emit(Instruction::DynamicNegate, span);
                    } else if let Some(num_kind) = to_numeric_kind(sem_type) {
                        self.emit(Instruction::Negate(num_kind), span);
                    }
                }
            },
            SemanticExpressionKind::Binary {
                left,
                operator,
                right,
            } => match operator {
                BinaryOperator::And => {
                    self.lower_expression(left);
                    let jump_false = self.emit_placeholder_jump_if_false(span);
                    self.lower_expression(right);
                    let jump_end = self.emit_placeholder_jump(span);

                    let false_label = self.instructions.len();
                    self.patch_jump(jump_false, false_label);
                    self.emit_load_constant(Constant::Boolean(false), span);

                    let end_label = self.instructions.len();
                    self.patch_jump(jump_end, end_label);
                }
                BinaryOperator::Or => {
                    self.lower_expression(left);
                    let jump_false = self.emit_placeholder_jump_if_false(span);
                    self.emit_load_constant(Constant::Boolean(true), span);
                    let jump_end = self.emit_placeholder_jump(span);

                    let false_label = self.instructions.len();
                    self.patch_jump(jump_false, false_label);
                    self.lower_expression(right);

                    let end_label = self.instructions.len();
                    self.patch_jump(jump_end, end_label);
                }
                BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Remainder => {
                    if matches!(sem_type, SemanticType::Native(NativeType::Dynamic)) {
                        self.lower_expression(left);
                        let l_type = &self.compiler.semantic_program.types[left.type_id.0];
                        if let Some(l_num) = to_numeric_kind(l_type) {
                            self.emit(Instruction::LiftDynamic(l_num), left.span);
                        }

                        self.lower_expression(right);
                        let r_type = &self.compiler.semantic_program.types[right.type_id.0];
                        if let Some(r_num) = to_numeric_kind(r_type) {
                            self.emit(Instruction::LiftDynamic(r_num), right.span);
                        }

                        match operator {
                            BinaryOperator::Add => self.emit(Instruction::DynamicAdd, span),
                            BinaryOperator::Subtract => {
                                self.emit(Instruction::DynamicSubtract, span)
                            }
                            BinaryOperator::Multiply => {
                                self.emit(Instruction::DynamicMultiply, span)
                            }
                            BinaryOperator::Divide => self.emit(Instruction::DynamicDivide, span),
                            BinaryOperator::Remainder => {
                                self.emit(Instruction::DynamicRemainder, span)
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        self.lower_expression(left);
                        self.lower_expression(right);
                        let num_kind = to_numeric_kind(sem_type).expect("fixed numeric operator");
                        match operator {
                            BinaryOperator::Add => self.emit(Instruction::Add(num_kind), span),
                            BinaryOperator::Subtract => {
                                self.emit(Instruction::Subtract(num_kind), span)
                            }
                            BinaryOperator::Multiply => {
                                self.emit(Instruction::Multiply(num_kind), span)
                            }
                            BinaryOperator::Divide => {
                                self.emit(Instruction::Divide(num_kind), span)
                            }
                            BinaryOperator::Remainder => {
                                self.emit(Instruction::Remainder(num_kind), span)
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                BinaryOperator::Equal | BinaryOperator::NotEqual => {
                    self.lower_expression(left);
                    self.lower_expression(right);
                    let op_tid = left.type_id.0;
                    let op_type = &self.compiler.semantic_program.types[op_tid];

                    match op_type {
                        SemanticType::Native(nt) => match nt {
                            NativeType::Bool => match operator {
                                BinaryOperator::Equal => self.emit(Instruction::EqualBoolean, span),
                                BinaryOperator::NotEqual => {
                                    self.emit(Instruction::NotEqualBoolean, span)
                                }
                                _ => unreachable!(),
                            },
                            NativeType::String => match operator {
                                BinaryOperator::Equal => self.emit(Instruction::EqualString, span),
                                BinaryOperator::NotEqual => {
                                    self.emit(Instruction::NotEqualString, span)
                                }
                                _ => unreachable!(),
                            },
                            _ => {
                                let num_kind =
                                    to_numeric_kind(op_type).expect("comparable numeric type");
                                match operator {
                                    BinaryOperator::Equal => {
                                        self.emit(Instruction::EqualNumeric(num_kind), span)
                                    }
                                    BinaryOperator::NotEqual => {
                                        self.emit(Instruction::NotEqualNumeric(num_kind), span)
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        },
                        SemanticType::Struct { .. } | SemanticType::Enum { .. } => {
                            let plan = self.compiler.build_composite_equality_plan(op_tid);
                            match operator {
                                BinaryOperator::Equal => {
                                    self.emit(Instruction::EqualComposite(plan), span)
                                }
                                BinaryOperator::NotEqual => {
                                    self.emit(Instruction::NotEqualComposite(plan), span)
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                BinaryOperator::Less
                | BinaryOperator::LessEqual
                | BinaryOperator::Greater
                | BinaryOperator::GreaterEqual => {
                    self.lower_expression(left);
                    self.lower_expression(right);
                    let op_type = &self.compiler.semantic_program.types[left.type_id.0];
                    let num_kind = to_numeric_kind(op_type).expect("orderable numeric type");
                    match operator {
                        BinaryOperator::Less => self.emit(Instruction::LessNumeric(num_kind), span),
                        BinaryOperator::LessEqual => {
                            self.emit(Instruction::LessEqualNumeric(num_kind), span)
                        }
                        BinaryOperator::Greater => {
                            self.emit(Instruction::GreaterNumeric(num_kind), span)
                        }
                        BinaryOperator::GreaterEqual => {
                            self.emit(Instruction::GreaterEqualNumeric(num_kind), span)
                        }
                        _ => unreachable!(),
                    }
                }
            },
            SemanticExpressionKind::Conversion { operand } => {
                self.lower_expression(operand);
                let src_tid = operand.type_id.0;
                let tgt_tid = expr_tid;
                let src_type = &self.compiler.semantic_program.types[src_tid];
                let tgt_type = &self.compiler.semantic_program.types[tgt_tid];

                if matches!(tgt_type, SemanticType::Native(NativeType::String)) {
                    if matches!(src_type, SemanticType::Native(NativeType::Dynamic)) {
                        self.emit(Instruction::DynamicToString, span);
                    } else if let Some(src_num) = to_numeric_kind(src_type) {
                        self.emit(Instruction::NumericToString(src_num), span);
                    }
                } else if matches!(src_type, SemanticType::Native(NativeType::Dynamic)) {
                    if let Some(tgt_num) = to_numeric_kind(tgt_type) {
                        self.emit(Instruction::ConvertDynamic(tgt_num), span);
                    }
                } else if let (Some(src_num), Some(tgt_num)) =
                    (to_numeric_kind(src_type), to_numeric_kind(tgt_type))
                {
                    if !numeric_kinds_match(&src_num, &tgt_num) {
                        self.emit(
                            Instruction::ConvertNumeric {
                                source: src_num,
                                target: tgt_num,
                            },
                            span,
                        );
                    }
                }
            }
            SemanticExpressionKind::FieldAccess { receiver, field } => {
                self.lower_expression(receiver);
                self.emit(Instruction::GetField(FieldIndex(field.0)), span);
            }
            SemanticExpressionKind::Call(call) => match &call.target {
                SemanticCallTarget::Internal(fid) => {
                    let target_func = &self.compiler.semantic_program.functions[fid.0];
                    let mut param_val_idx = 0;
                    for arg in &call.arguments {
                        match arg {
                            SemanticArgument::Value(arg_expr) => {
                                let mut expected_tid = arg_expr.type_id.0;
                                let mut count = 0;
                                for p in &target_func.parameters {
                                    if let SemanticParameter::Value(bid) = p {
                                        if count == param_val_idx {
                                            expected_tid = target_func.bindings[bid.0].type_id.0;
                                            break;
                                        }
                                        count += 1;
                                    }
                                }
                                param_val_idx += 1;
                                self.lower_expression(arg_expr);
                                self.adapt_to_expected_type(
                                    arg_expr.type_id.0,
                                    expected_tid,
                                    arg_expr.span,
                                );
                            }
                            SemanticArgument::SignatureDependency(_) => {}
                        }
                    }
                    self.emit(Instruction::Call(FunctionId(fid.0)), span);
                }
                SemanticCallTarget::DirectSignature(sid) => {
                    let sig = &self.compiler.semantic_program.signatures[sid.0];
                    let esid_idx = self.compiler.get_or_create_external_symbol(sig);
                    let mut param_val_idx = 0;
                    for arg in &call.arguments {
                        match arg {
                            SemanticArgument::Value(arg_expr) => {
                                let mut expected_tid = arg_expr.type_id.0;
                                let mut count = 0;
                                for p in &sig.parameters {
                                    if let SemanticSignatureParameter::Value(tid) = p {
                                        if count == param_val_idx {
                                            expected_tid = tid.0;
                                            break;
                                        }
                                        count += 1;
                                    }
                                }
                                param_val_idx += 1;
                                self.lower_expression(arg_expr);
                                self.adapt_to_expected_type(
                                    arg_expr.type_id.0,
                                    expected_tid,
                                    arg_expr.span,
                                );
                            }
                            SemanticArgument::SignatureDependency(_) => {}
                        }
                    }
                    self.emit(Instruction::CallExternal(ExternalSymbolId(esid_idx)), span);
                }
                SemanticCallTarget::SignatureDependency(sbid) => {
                    let sid_idx = self.function.signature_bindings[sbid.0].signature.0;
                    let sig = &self.compiler.semantic_program.signatures[sid_idx];
                    let esid_idx = self.compiler.get_or_create_external_symbol(sig);
                    let mut param_val_idx = 0;
                    for arg in &call.arguments {
                        match arg {
                            SemanticArgument::Value(arg_expr) => {
                                let mut expected_tid = arg_expr.type_id.0;
                                let mut count = 0;
                                for p in &sig.parameters {
                                    if let SemanticSignatureParameter::Value(tid) = p {
                                        if count == param_val_idx {
                                            expected_tid = tid.0;
                                            break;
                                        }
                                        count += 1;
                                    }
                                }
                                param_val_idx += 1;
                                self.lower_expression(arg_expr);
                                self.adapt_to_expected_type(
                                    arg_expr.type_id.0,
                                    expected_tid,
                                    arg_expr.span,
                                );
                            }
                            SemanticArgument::SignatureDependency(_) => {}
                        }
                    }
                    self.emit(Instruction::CallExternal(ExternalSymbolId(esid_idx)), span);
                }
            },
            SemanticExpressionKind::StructConstruction { fields } => {
                let struct_type = match sem_type {
                    SemanticType::Struct { fields } => fields,
                    _ => panic!("expected struct type"),
                };

                let mut field_order = Vec::with_capacity(fields.len());
                for f in fields {
                    let expected_tid = struct_type[f.field.0].type_id.0;
                    self.lower_expression(&f.value);
                    self.adapt_to_expected_type(f.value.type_id.0, expected_tid, f.value.span);
                    field_order.push(FieldIndex(f.field.0));
                }

                self.emit(Instruction::ConstructStruct { field_order }, span);
            }
            SemanticExpressionKind::EnumConstruction { variant, payload } => match payload {
                SemanticEnumPayload::Simple => {
                    self.emit(
                        Instruction::ConstructEnumSimple(VariantDiscriminant(variant.0)),
                        span,
                    );
                }
                SemanticEnumPayload::Associated { value } => {
                    let enum_variants = match sem_type {
                        SemanticType::Enum { variants } => variants,
                        _ => panic!("expected enum type"),
                    };
                    let expected_tid = match &enum_variants[variant.0] {
                        SemanticVariant::Associated { type_id } => type_id.0,
                        _ => panic!("expected associated variant"),
                    };
                    self.lower_expression(value);
                    self.adapt_to_expected_type(value.type_id.0, expected_tid, value.span);
                    self.emit(
                        Instruction::ConstructEnumAssociated(VariantDiscriminant(variant.0)),
                        span,
                    );
                }
                SemanticEnumPayload::Structured { fields } => {
                    let enum_variants = match sem_type {
                        SemanticType::Enum { variants } => variants,
                        _ => panic!("expected enum type"),
                    };
                    let def_fields = match &enum_variants[variant.0] {
                        SemanticVariant::Structured { fields } => fields,
                        _ => panic!("expected structured variant"),
                    };

                    let mut field_order = Vec::with_capacity(fields.len());
                    for f in fields {
                        let expected_tid = def_fields[f.field.0].type_id.0;
                        self.lower_expression(&f.value);
                        self.adapt_to_expected_type(f.value.type_id.0, expected_tid, f.value.span);
                        field_order.push(FieldIndex(f.field.0));
                    }

                    self.emit(
                        Instruction::ConstructEnumStructured {
                            variant: VariantDiscriminant(variant.0),
                            field_order,
                        },
                        span,
                    );
                }
            },
            SemanticExpressionKind::When(when) => {
                self.lower_expression(&when.subject);
                let when_tid = expr_tid;
                let mut jump_ends = Vec::new();
                let num_branches = when.branches.len();

                for (branch_idx, branch) in when.branches.iter().enumerate() {
                    let is_last = branch_idx == num_branches - 1;

                    if is_last {
                        // Fallback branch: subject is on top of stack
                        match &branch.extraction {
                            SemanticVariantExtraction::Simple => {
                                self.emit(Instruction::Discard, span);
                            }
                            SemanticVariantExtraction::Associated { binding } => {
                                self.emit(Instruction::ExtractEnumAssociated, span);
                                let slot_idx = match self.binding_to_slot[&binding.0] {
                                    Slot::Local(l) => l,
                                    Slot::Parameter(_) => {
                                        panic!("extraction cannot be parameter slot")
                                    }
                                };
                                self.emit(Instruction::StoreLocal(LocalSlot(slot_idx)), span);
                            }
                            SemanticVariantExtraction::Structured { fields } => {
                                let field_indices: Vec<FieldIndex> =
                                    fields.iter().map(|f| FieldIndex(f.field.0)).collect();
                                self.emit(
                                    Instruction::ExtractEnumStructured {
                                        fields: field_indices,
                                    },
                                    span,
                                );
                                // Store in reverse order because stack is LIFO
                                for f_binding in fields.iter().rev() {
                                    let slot_idx = match self.binding_to_slot[&f_binding.binding.0]
                                    {
                                        Slot::Local(l) => l,
                                        Slot::Parameter(_) => {
                                            panic!("extraction cannot be parameter slot")
                                        }
                                    };
                                    self.emit(Instruction::StoreLocal(LocalSlot(slot_idx)), span);
                                }
                            }
                        }
                        self.lower_expression(&branch.result);
                        self.adapt_to_expected_type(
                            branch.result.type_id.0,
                            when_tid,
                            branch.result.span,
                        );
                    } else {
                        // Non-last branch: TestVariant consumes subject and pushes [subject, match_bool]
                        self.emit(
                            Instruction::TestVariant(VariantDiscriminant(branch.variant.0)),
                            span,
                        );
                        let jump_next = self.emit_placeholder_jump_if_false(span);

                        // Match branch: subject is on top of stack
                        match &branch.extraction {
                            SemanticVariantExtraction::Simple => {
                                self.emit(Instruction::Discard, span);
                            }
                            SemanticVariantExtraction::Associated { binding } => {
                                self.emit(Instruction::ExtractEnumAssociated, span);
                                let slot_idx = match self.binding_to_slot[&binding.0] {
                                    Slot::Local(l) => l,
                                    Slot::Parameter(_) => {
                                        panic!("extraction cannot be parameter slot")
                                    }
                                };
                                self.emit(Instruction::StoreLocal(LocalSlot(slot_idx)), span);
                            }
                            SemanticVariantExtraction::Structured { fields } => {
                                let field_indices: Vec<FieldIndex> =
                                    fields.iter().map(|f| FieldIndex(f.field.0)).collect();
                                self.emit(
                                    Instruction::ExtractEnumStructured {
                                        fields: field_indices,
                                    },
                                    span,
                                );
                                for f_binding in fields.iter().rev() {
                                    let slot_idx = match self.binding_to_slot[&f_binding.binding.0]
                                    {
                                        Slot::Local(l) => l,
                                        Slot::Parameter(_) => {
                                            panic!("extraction cannot be parameter slot")
                                        }
                                    };
                                    self.emit(Instruction::StoreLocal(LocalSlot(slot_idx)), span);
                                }
                            }
                        }
                        self.lower_expression(&branch.result);
                        self.adapt_to_expected_type(
                            branch.result.type_id.0,
                            when_tid,
                            branch.result.span,
                        );

                        let jump_end = self.emit_placeholder_jump(span);
                        jump_ends.push(jump_end);

                        let next_label = self.instructions.len();
                        self.patch_jump(jump_next, next_label);
                    }
                }

                let end_label = self.instructions.len();
                for j_end in jump_ends {
                    self.patch_jump(j_end, end_label);
                }
            }
        }
    }
}

// --- Helper Functions ---

fn lower_literal(lit: &SemanticLiteral, sem_type: &SemanticType) -> Constant {
    match lit {
        SemanticLiteral::Boolean(b) => Constant::Boolean(*b),
        SemanticLiteral::String(s) => Constant::String(s.clone()),
        SemanticLiteral::Floating(f) => match sem_type {
            SemanticType::Native(NativeType::Float32) => Constant::Float32(*f as f32),
            SemanticType::Native(NativeType::Float | NativeType::Float64) => Constant::Float64(*f),
            SemanticType::Native(NativeType::Dynamic) => {
                Constant::Dynamic(DynamicConstant::Float64(*f))
            }
            _ => Constant::Float64(*f),
        },
        SemanticLiteral::Integer(s) => match sem_type {
            SemanticType::Native(nt) => match nt {
                NativeType::Int8 => {
                    let val = s
                        .parse::<i8>()
                        .unwrap_or_else(|_| if s == "128" { -128 } else { 0 });
                    Constant::Int8(val)
                }
                NativeType::Int16 => {
                    let val = s
                        .parse::<i16>()
                        .unwrap_or_else(|_| if s == "32768" { -32768 } else { 0 });
                    Constant::Int16(val)
                }
                NativeType::Int | NativeType::Int32 => {
                    let val = s
                        .parse::<i32>()
                        .unwrap_or_else(|_| if s == "2147483648" { -2147483648 } else { 0 });
                    Constant::Int32(val)
                }
                NativeType::Int64 => {
                    let val = s.parse::<i64>().unwrap_or_else(|_| {
                        if s == "9223372036854775808" {
                            -9223372036854775808
                        } else {
                            0
                        }
                    });
                    Constant::Int64(val)
                }
                NativeType::Int128 => {
                    let val = s.parse::<i128>().unwrap_or_else(|_| {
                        if s == "170141183460469231731687303715884105728" {
                            -170141183460469231731687303715884105728
                        } else {
                            0
                        }
                    });
                    Constant::Int128(val)
                }
                NativeType::Uint8 => Constant::Uint8(s.parse::<u8>().unwrap_or(0)),
                NativeType::Uint16 => Constant::Uint16(s.parse::<u16>().unwrap_or(0)),
                NativeType::Uint32 => Constant::Uint32(s.parse::<u32>().unwrap_or(0)),
                NativeType::Uint64 => Constant::Uint64(s.parse::<u64>().unwrap_or(0)),
                NativeType::Uint128 => Constant::Uint128(s.parse::<u128>().unwrap_or(0)),
                NativeType::Dynamic => {
                    let magnitude = decimal_string_to_big_endian_magnitude(s);
                    Constant::Dynamic(DynamicConstant::Integer {
                        negative: false,
                        magnitude,
                    })
                }
                NativeType::Float | NativeType::Float64 => {
                    let val = s.parse::<f64>().unwrap_or(0.0);
                    Constant::Float64(val)
                }
                NativeType::Float32 => {
                    let val = s.parse::<f32>().unwrap_or(0.0);
                    Constant::Float32(val)
                }
                _ => Constant::Int32(s.parse::<i32>().unwrap_or(0)),
            },
            _ => Constant::Int32(s.parse::<i32>().unwrap_or(0)),
        },
    }
}

fn decimal_string_to_big_endian_magnitude(s: &str) -> Vec<u8> {
    let s = s.trim_start_matches('0');
    if s.is_empty() {
        return Vec::new();
    }
    let mut bytes: Vec<u8> = Vec::new();
    for ch in s.chars() {
        if let Some(digit) = ch.to_digit(10) {
            let mut carry = digit as u16;
            for byte in bytes.iter_mut().rev() {
                let cur = (*byte as u16) * 10 + carry;
                *byte = (cur & 0xFF) as u8;
                carry = cur >> 8;
            }
            while carry > 0 {
                bytes.insert(0, (carry & 0xFF) as u8);
                carry >>= 8;
            }
        }
    }
    let first_non_zero = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    bytes[first_non_zero..].to_vec()
}

fn to_numeric_kind(sem_type: &SemanticType) -> Option<NumericKind> {
    match sem_type {
        SemanticType::Native(nt) => match nt {
            NativeType::Int | NativeType::Int32 => Some(NumericKind::Int32),
            NativeType::Int8 => Some(NumericKind::Int8),
            NativeType::Int16 => Some(NumericKind::Int16),
            NativeType::Int64 => Some(NumericKind::Int64),
            NativeType::Int128 => Some(NumericKind::Int128),
            NativeType::Uint8 => Some(NumericKind::Uint8),
            NativeType::Uint16 => Some(NumericKind::Uint16),
            NativeType::Uint32 => Some(NumericKind::Uint32),
            NativeType::Uint64 => Some(NumericKind::Uint64),
            NativeType::Uint128 => Some(NumericKind::Uint128),
            NativeType::Float | NativeType::Float64 => Some(NumericKind::Float64),
            NativeType::Float32 => Some(NumericKind::Float32),
            _ => None,
        },
        _ => None,
    }
}

fn numeric_kinds_match(a: &NumericKind, b: &NumericKind) -> bool {
    matches!(
        (a, b),
        (NumericKind::Int8, NumericKind::Int8)
            | (NumericKind::Int16, NumericKind::Int16)
            | (NumericKind::Int32, NumericKind::Int32)
            | (NumericKind::Int64, NumericKind::Int64)
            | (NumericKind::Int128, NumericKind::Int128)
            | (NumericKind::Uint8, NumericKind::Uint8)
            | (NumericKind::Uint16, NumericKind::Uint16)
            | (NumericKind::Uint32, NumericKind::Uint32)
            | (NumericKind::Uint64, NumericKind::Uint64)
            | (NumericKind::Uint128, NumericKind::Uint128)
            | (NumericKind::Float32, NumericKind::Float32)
            | (NumericKind::Float64, NumericKind::Float64)
    )
}

fn compute_max_operand_depth(
    instructions: &[Instruction],
    compiled_functions_param_counts: &[usize],
    external_symbols_param_counts: &[usize],
) -> usize {
    if instructions.is_empty() {
        return 0;
    }

    let n = instructions.len();
    let mut depths: Vec<Option<usize>> = vec![None; n];
    let mut max_depth = 0;

    let mut queue = VecDeque::new();
    depths[0] = Some(0);
    queue.push_back(0);

    while let Some(ip) = queue.pop_front() {
        let current_depth = depths[ip].unwrap();
        let instr = &instructions[ip];

        let (consumed, produced) = match instr {
            Instruction::LoadConstant(_)
            | Instruction::LoadParameter(_)
            | Instruction::LoadLocal(_)
            | Instruction::ConstructEnumSimple(_) => (0, 1),

            Instruction::StoreLocal(_)
            | Instruction::JumpIfFalse(_)
            | Instruction::Discard
            | Instruction::Return => (1, 0),

            Instruction::Call(fid) => {
                let p_count = compiled_functions_param_counts[fid.0];
                (p_count, 1)
            }
            Instruction::CallExternal(esid) => {
                let p_count = external_symbols_param_counts[esid.0];
                (p_count, 1)
            }

            Instruction::Negate(_)
            | Instruction::LiftDynamic(_)
            | Instruction::DynamicNegate
            | Instruction::ConvertNumeric { .. }
            | Instruction::ConvertDynamic(_)
            | Instruction::NumericToString(_)
            | Instruction::DynamicToString
            | Instruction::NotBoolean
            | Instruction::GetField(_)
            | Instruction::ConstructEnumAssociated(_)
            | Instruction::ExtractEnumAssociated => (1, 1),

            Instruction::Add(_)
            | Instruction::Subtract(_)
            | Instruction::Multiply(_)
            | Instruction::Divide(_)
            | Instruction::Remainder(_)
            | Instruction::EqualNumeric(_)
            | Instruction::NotEqualNumeric(_)
            | Instruction::LessNumeric(_)
            | Instruction::LessEqualNumeric(_)
            | Instruction::GreaterNumeric(_)
            | Instruction::GreaterEqualNumeric(_)
            | Instruction::DynamicAdd
            | Instruction::DynamicSubtract
            | Instruction::DynamicMultiply
            | Instruction::DynamicDivide
            | Instruction::DynamicRemainder
            | Instruction::EqualBoolean
            | Instruction::NotEqualBoolean
            | Instruction::EqualString
            | Instruction::NotEqualString
            | Instruction::EqualComposite(_)
            | Instruction::NotEqualComposite(_) => (2, 1),

            Instruction::Jump(_) => (0, 0),

            Instruction::ConstructStruct { field_order } => (field_order.len(), 1),
            Instruction::ConstructEnumStructured { field_order, .. } => (field_order.len(), 1),

            Instruction::TestVariant(_) => (1, 2),
            Instruction::ExtractEnumStructured { fields } => (1, fields.len()),
        };

        if current_depth > max_depth {
            max_depth = current_depth;
        }

        assert!(
            current_depth >= consumed,
            "Stack underflow at instruction {}: depth {}, consumed {}",
            ip,
            current_depth,
            consumed
        );

        let depth_after = current_depth - consumed + produced;
        if depth_after > max_depth {
            max_depth = depth_after;
        }

        let mut successors = Vec::new();
        match instr {
            Instruction::Jump(target) => {
                successors.push(target.0);
            }
            Instruction::JumpIfFalse(target) => {
                successors.push(target.0);
                if ip + 1 < n {
                    successors.push(ip + 1);
                }
            }
            Instruction::Return => {}
            _ => {
                if ip + 1 < n {
                    successors.push(ip + 1);
                }
            }
        }

        for next_ip in successors {
            if next_ip < n {
                if let Some(existing_depth) = depths[next_ip] {
                    assert_eq!(
                        existing_depth, depth_after,
                        "Stack depth mismatch at instruction {}: {} vs {}",
                        next_ip, existing_depth, depth_after
                    );
                } else {
                    depths[next_ip] = Some(depth_after);
                    queue.push_back(next_ip);
                }
            }
        }
    }

    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collaborators::lexer::lex_source;
    use crate::collaborators::parser::parse_tokens;
    use crate::collaborators::semantic_analyzer::analyze_program;
    use crate::data::compilation_dependency::{
        CatalogSignature, CatalogSignatureParameter, CatalogTypeRef, CompilationCatalog,
    };

    fn compile_src(src: &str, catalog: &CompilationCatalog) -> CompiledProgram {
        let tokens = lex_source(src).unwrap_or_else(|_| panic!("lexing failed"));
        let program = parse_tokens(&tokens, src).unwrap_or_else(|_| panic!("parsing failed"));
        let semantic_program = analyze_program(&program, catalog)
            .unwrap_or_else(|_| panic!("semantic analysis failed"));
        lower_program(&semantic_program)
    }

    #[test]
    fn typed_lower_signature_and_binding() {
        let lower_fn: Lower = lower_program;
        let lower_const: Lower = LOWER_PROGRAM;
        assert_eq!(lower_fn as usize, lower_const as usize);
    }

    #[test]
    fn basic_function_compilation_and_owned_independence() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return 42; }";
        let compiled = compile_src(src, &cat);

        assert_eq!(compiled.functions.len(), 1);
        assert_eq!(compiled.entry_point.0, 0);
        assert_eq!(compiled.entry_parameter_shapes.len(), 0);
        assert_eq!(compiled.functions[0].parameter_count, 0);
        assert_eq!(compiled.functions[0].local_count, 0);
        assert!(compiled.functions[0].max_operand_depth >= 1);
        assert_eq!(compiled.source_map.functions.len(), 1);
        assert_eq!(
            compiled.source_map.functions[0].len(),
            compiled.functions[0].instructions.len()
        );
    }

    #[test]
    fn parameters_and_locals_and_let_bindings() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main(int8 a, int8 b) -> int8 {
                let int8 sum = a + b;
                return sum;
            }
        "#;
        let compiled = compile_src(src, &cat);

        assert_eq!(compiled.functions[0].parameter_count, 2);
        assert_eq!(compiled.functions[0].local_count, 1);
        assert_eq!(compiled.entry_parameter_shapes.len(), 2);
    }

    #[test]
    fn signature_dependency_parameter_erased_physically() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "dep".to_string(),
                name: "Op".to_string(),
            },
            CatalogSignature {
                parameters: vec![CatalogSignatureParameter::Value(CatalogTypeRef::Int8)],
                result_type: CatalogTypeRef::Int8,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };

        let src = r#"
            import dep::Op;

            public fn main(int8 val, dep::Op op) -> int8 {
                return op(val);
            }
        "#;
        let compiled = compile_src(src, &cat);

        // Op is a signature dependency, so parameter_count must be 1 (val only)
        assert_eq!(compiled.functions[0].parameter_count, 1);
        assert_eq!(compiled.entry_parameter_shapes.len(), 1);
        assert_eq!(compiled.external_symbols.len(), 1);
        assert_eq!(compiled.external_symbols[0].parameter_count, 1);
    }

    #[test]
    fn large_dynamic_integer_greater_than_u128() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main() -> dynamic {
                return 34028236692093846346337460743176821145600000;
            }
        "#;
        let compiled = compile_src(src, &cat);
        assert_eq!(compiled.constants.len(), 1);
        match &compiled.constants[0] {
            Constant::Dynamic(DynamicConstant::Integer {
                negative,
                magnitude,
            }) => {
                assert!(!negative);
                assert!(!magnitude.is_empty());
            }
            _ => panic!("expected dynamic integer constant"),
        }
    }

    #[test]
    fn signed_minimum_constants_no_overflow() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main() -> int8 {
                return -128;
            }
        "#;
        let compiled = compile_src(src, &cat);
        assert_eq!(compiled.constants.len(), 1);
        match compiled.constants[0] {
            Constant::Int8(v) => assert_eq!(v, -128),
            _ => panic!("expected Int8(-128)"),
        }
    }

    #[test]
    fn fixed_and_dynamic_arithmetic_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src_fixed = "public fn main(int8 a, int8 b) -> int8 { return a + b; }";
        let compiled_fixed = compile_src(src_fixed, &cat);
        let has_add_int8 = compiled_fixed.functions[0]
            .instructions
            .iter()
            .any(|instr| matches!(instr, Instruction::Add(NumericKind::Int8)));
        assert!(has_add_int8);

        let src_dynamic = "public fn main(int8 a, int8 b) -> dynamic { return a + b; }";
        let compiled_dyn = compile_src(src_dynamic, &cat);
        let has_dyn_add = compiled_dyn.functions[0]
            .instructions
            .iter()
            .any(|instr| matches!(instr, Instruction::DynamicAdd));
        let lift_count = compiled_dyn.functions[0]
            .instructions
            .iter()
            .filter(|instr| matches!(instr, Instruction::LiftDynamic(NumericKind::Int8)))
            .count();
        assert!(has_dyn_add);
        assert_eq!(lift_count, 2);
    }

    #[test]
    fn concrete_call_result_lifted_only_after_call() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            private fn calc(int8 a, int8 b) -> int8 {
                return a + b;
            }

            public fn main(int8 a, int8 b) -> dynamic {
                return calc(a, b);
            }
        "#;
        let compiled = compile_src(src, &cat);
        // calc should have Add(Int8) without LiftDynamic
        let calc_has_lift = compiled.functions[0]
            .instructions
            .iter()
            .any(|i| matches!(i, Instruction::LiftDynamic(_)));
        assert!(!calc_has_lift);

        // main should have Call(0) followed by LiftDynamic(Int8)
        let main_instrs = &compiled.functions[1].instructions;
        let call_idx = main_instrs
            .iter()
            .position(|i| matches!(i, Instruction::Call(FunctionId(0))))
            .unwrap();
        assert!(matches!(
            main_instrs[call_idx + 1],
            Instruction::LiftDynamic(NumericKind::Int8)
        ));
    }

    #[test]
    fn boolean_short_circuit_and_or() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main(bool a, bool b, bool c) -> bool {
                let bool r1 = a && b;
                let bool r2 = a || c;
                return r1 && r2;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let has_jump_if_false = compiled.functions[0]
            .instructions
            .iter()
            .any(|i| matches!(i, Instruction::JumpIfFalse(_)));
        assert!(has_jump_if_false);
    }

    #[test]
    fn to_string_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src_num = "public fn main(int8 val) -> string { return to_string(val); }";
        let comp_num = compile_src(src_num, &cat);
        assert!(
            comp_num.functions[0]
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::NumericToString(NumericKind::Int8)))
        );

        let src_dyn = "public fn main(dynamic val) -> string { return to_string(val); }";
        let comp_dyn = compile_src(src_dyn, &cat);
        assert!(
            comp_dyn.functions[0]
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::DynamicToString))
        );
    }

    #[test]
    fn struct_construction_evaluation_order_and_canonical_field_order() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            struct Worker {
                int age;
                string name;
            }

            private fn get_name() -> string { return "Alice"; }
            private fn get_age() -> int { return 30; }

            public fn main() -> Worker {
                return Worker {
                    name: get_name(),
                    age: get_age()
                };
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = compiled.functions.last().unwrap();

        // Check call order: get_name (0) then get_age (1)
        let mut call_targets = Vec::new();
        for instr in &main_fn.instructions {
            if let Instruction::Call(fid) = instr {
                call_targets.push(fid.0);
            }
        }
        assert_eq!(call_targets, vec![0, 1]);

        // ConstructStruct field_order must map source field 0 (name = FieldIndex(1)) and source field 1 (age = FieldIndex(0))
        let construct_instr = main_fn
            .instructions
            .iter()
            .find_map(|i| match i {
                Instruction::ConstructStruct { field_order } => Some(field_order),
                _ => None,
            })
            .unwrap();
        assert_eq!(construct_instr.len(), 2);
        assert_eq!(construct_instr[0].0, 1);
        assert_eq!(construct_instr[1].0, 0);
    }

    #[test]
    fn structured_enum_construction_order() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Event {
                Data {
                    int code;
                    string msg;
                }
            }

            private fn get_msg() -> string { return "err"; }
            private fn get_code() -> int { return 404; }

            public fn main() -> Event {
                return Event::Data {
                    msg: get_msg(),
                    code: get_code()
                };
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = compiled.functions.last().unwrap();

        let construct_instr = main_fn
            .instructions
            .iter()
            .find_map(|i| match i {
                Instruction::ConstructEnumStructured {
                    variant,
                    field_order,
                } => Some((variant.0, field_order)),
                _ => None,
            })
            .unwrap();
        assert_eq!(construct_instr.0, 0); // variant 0
        assert_eq!(construct_instr.1[0].0, 1); // msg is FieldIndex(1)
        assert_eq!(construct_instr.1[1].0, 0); // code is FieldIndex(0)
    }

    #[test]
    fn when_pattern_matching_lowering_and_single_subject_evaluation() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Shape {
                Circle(float),
                Point,
                Rect { float w; float h; }
            }

            public fn main(Shape s) -> float {
                return when s {
                    Shape::Point => 0.0,
                    Shape::Circle(float r) => r,
                    Shape::Rect { w: float w; h: float h; } => w * h
                };
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];

        // Subject loaded once
        let param_load_count = main_fn
            .instructions
            .iter()
            .filter(|i| matches!(i, Instruction::LoadParameter(ParameterSlot(0))))
            .count();
        assert_eq!(param_load_count, 1);

        // Contains TestVariant and extractions
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::TestVariant(_)))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::ExtractEnumAssociated))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::ExtractEnumStructured { .. }))
        );
    }

    #[test]
    fn structural_equality_composite_plan_generation() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            struct Point {
                int x;
                int y;
            }

            public fn main(Point a, Point b) -> bool {
                return a == b;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];

        let has_composite_eq = main_fn
            .instructions
            .iter()
            .any(|i| matches!(i, Instruction::EqualComposite(_)));
        assert!(has_composite_eq);
    }

    #[test]
    fn direct_signature_call_external() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "math".to_string(),
                name: "sqrt".to_string(),
            },
            CatalogSignature {
                parameters: vec![CatalogSignatureParameter::Value(CatalogTypeRef::Float)],
                result_type: CatalogTypeRef::Float,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };

        let src = r#"
            import math::sqrt;

            public fn main(float val) -> float {
                return sqrt(val);
            }
        "#;
        let compiled = compile_src(src, &cat);
        assert_eq!(compiled.external_symbols.len(), 1);
        assert_eq!(compiled.external_symbols[0].symbol.module, "math");
        assert_eq!(compiled.external_symbols[0].symbol.name, "sqrt");
        assert_eq!(compiled.external_symbols[0].parameter_count, 1);

        let main_fn = &compiled.functions[0];
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::CallExternal(ExternalSymbolId(0))))
        );
    }

    #[test]
    fn all_scalar_literal_constants() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main() -> bool {
                let int8 i8_val = 10;
                let int16 i16_val = 20;
                let int32 i32_val = 30;
                let int64 i64_val = 40;
                let int128 i128_val = 50;
                let uint8 u8_val = 60;
                let uint16 u16_val = 70;
                let uint32 u32_val = 80;
                let uint64 u64_val = 90;
                let uint128 u128_val = 100;
                let float32 f32_val = 1.5;
                let float64 f64_val = 2.5;
                let string s_val = "text";
                let bool b_val = true;
                return b_val;
            }
        "#;
        let compiled = compile_src(src, &cat);
        assert!(compiled.constants.len() >= 14);
    }

    #[test]
    fn nested_field_access_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            struct Inner {
                int val;
            }
            struct Outer {
                Inner inner;
            }

            public fn main(Outer o) -> int {
                return o.inner.val;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];
        let get_fields: Vec<usize> = main_fn
            .instructions
            .iter()
            .filter_map(|i| match i {
                Instruction::GetField(f) => Some(f.0),
                _ => None,
            })
            .collect();
        assert_eq!(get_fields, vec![0, 0]);
    }

    #[test]
    fn enum_equality_plan_generation() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Status {
                Active,
                Code(int),
                Info { int id; string msg; }
            }

            public fn main(Status a, Status b) -> bool {
                return a == b;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];
        assert!(main_fn.instructions.iter().any(|i| matches!(
            i,
            Instruction::EqualComposite(CompositeEqualityPlan::Enum { .. })
        )));
    }

    #[test]
    fn comparison_operators_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main(int a, int b) -> bool {
                let bool r1 = a < b;
                let bool r2 = a <= b;
                let bool r3 = a > b;
                let bool r4 = a >= b;
                let bool r5 = a == b;
                let bool r6 = a != b;
                return r1 && r2 && r3 && r4 && r5 && r6;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::LessNumeric(NumericKind::Int32)))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::LessEqualNumeric(NumericKind::Int32)))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::GreaterNumeric(NumericKind::Int32)))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::GreaterEqualNumeric(NumericKind::Int32)))
        );
    }

    #[test]
    fn source_map_density_and_spans() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return 10 + 20; }";
        let compiled = compile_src(src, &cat);
        assert_eq!(compiled.source_map.functions.len(), 1);
        assert_eq!(
            compiled.source_map.functions[0].len(),
            compiled.functions[0].instructions.len()
        );
        for span in &compiled.source_map.functions[0] {
            assert!(span.end >= span.start);
        }
    }

    #[test]
    fn when_source_map_provenance_exact_spans() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Status {
                Ready,
                Failed
            }

            public fn main(Status status) -> int {
                return when status {
                    Status::Ready => 10,
                    Status::Failed => 20
                };
            }
        "#;
        let tokens = lex_source(src).unwrap_or_else(|_| panic!("lexing failed"));
        let ast = parse_tokens(&tokens, src).unwrap_or_else(|_| panic!("parsing failed"));
        let sem = analyze_program(&ast, &cat).unwrap_or_else(|_| panic!("analysis failed"));
        let when_span = sem.functions[0].body.result.span;
        let compiled = lower_program(&sem);

        let _main_fn = &compiled.functions[0];
        let spans = &compiled.source_map.functions[0];

        // 0: LoadParameter(0) -> status parameter span
        // 1: TestVariant(0) -> when_span
        // 2: JumpIfFalse -> when_span
        // 3: Discard -> when_span
        // 4: LoadConstant(10) -> span of 10
        // 5: Jump(end) -> when_span
        // 6: Discard -> when_span
        // 7: LoadConstant(20) -> span of 20
        // 8: Return -> return result expression span (when_span)
        assert_eq!(spans[1], when_span);
        assert_eq!(spans[2], when_span);
        assert_eq!(spans[3], when_span);
        assert_ne!(spans[4], when_span);
        assert_eq!(spans[5], when_span);
        assert_eq!(spans[6], when_span);
        assert_ne!(spans[7], when_span);
        assert_eq!(spans[8], when_span);
    }

    #[test]
    fn when_extraction_source_map_provenance_associated_and_structured() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Result {
                Value(int),
                Data { int code; string msg; },
                Empty
            }

            public fn main(Result res) -> int {
                return when res {
                    Result::Value(int v) => v + 1,
                    Result::Data { code: int c; msg: string m; } => c,
                    Result::Empty => 0
                };
            }
        "#;
        let tokens = lex_source(src).unwrap_or_else(|_| panic!("lexing failed"));
        let ast = parse_tokens(&tokens, src).unwrap_or_else(|_| panic!("parsing failed"));
        let sem = analyze_program(&ast, &cat).unwrap_or_else(|_| panic!("analysis failed"));
        let when_span = sem.functions[0].body.result.span;
        let compiled = lower_program(&sem);

        let main_fn = &compiled.functions[0];
        let spans = &compiled.source_map.functions[0];

        // Find ExtractEnumAssociated and StoreLocal
        let extract_assoc_pos = main_fn
            .instructions
            .iter()
            .position(|i| matches!(i, Instruction::ExtractEnumAssociated))
            .unwrap();
        assert_eq!(spans[extract_assoc_pos], when_span);
        assert_eq!(spans[extract_assoc_pos + 1], when_span); // StoreLocal for associated extraction

        // Find ExtractEnumStructured and StoreLocals
        let extract_struct_pos = main_fn
            .instructions
            .iter()
            .position(|i| matches!(i, Instruction::ExtractEnumStructured { .. }))
            .unwrap();
        assert_eq!(spans[extract_struct_pos], when_span);
        assert_eq!(spans[extract_struct_pos + 1], when_span); // StoreLocal 1
        assert_eq!(spans[extract_struct_pos + 2], when_span); // StoreLocal 2
    }

    #[test]
    fn numeric_conversions_and_dynamic_conversion_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main(int8 a, dynamic d) -> int64 {
                let int64 c1 = to_int64(a);
                let int32 c2 = to_int32(d);
                return c1;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];

        assert!(main_fn.instructions.iter().any(|i| matches!(
            i,
            Instruction::ConvertNumeric {
                source: NumericKind::Int8,
                target: NumericKind::Int64,
            }
        )));

        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::ConvertDynamic(NumericKind::Int32)))
        );
    }

    #[test]
    fn boolean_not_and_string_equality_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main(bool flag, string s1, string s2) -> bool {
                let bool r1 = !flag;
                let bool r2 = s1 == s2;
                let bool r3 = s1 != s2;
                return r1 && r2 && r3;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];

        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::NotBoolean))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::EqualString))
        );
        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::NotEqualString))
        );
    }

    #[test]
    fn simple_and_associated_enum_construction_lowering() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Status {
                Active,
                WithId(int)
            }

            public fn main() -> Status {
                let Status s1 = Status::Active;
                let Status s2 = Status::WithId(42);
                return s1;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];

        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::ConstructEnumSimple(VariantDiscriminant(0))))
        );
        assert!(main_fn.instructions.iter().any(|i| matches!(
            i,
            Instruction::ConstructEnumAssociated(VariantDiscriminant(1))
        )));
    }

    #[test]
    fn composite_not_equal_plan_generation() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            struct Point {
                int x;
                int y;
            }

            public fn main(Point a, Point b) -> bool {
                return a != b;
            }
        "#;
        let compiled = compile_src(src, &cat);
        let main_fn = &compiled.functions[0];

        assert!(
            main_fn
                .instructions
                .iter()
                .any(|i| matches!(i, Instruction::NotEqualComposite(_)))
        );
    }
}
