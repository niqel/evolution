use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use std::collections::{HashMap, HashSet};

use crate::data::ast::expressions::{
    BinaryOperator, EnumConstruction, Expression, ExpressionKind, FieldInitializer, FunctionCall,
    LiteralKind, Pipeline, UnaryOperator,
};
use crate::data::ast::foundational::{Identifier, QualifiedName, Visibility};
use crate::data::ast::functions::{
    BodyStatement, FunctionDefinition, OperationStatement, Parameter,
};
use crate::data::ast::local_types::EnumVariant;
use crate::data::ast::program::{Declaration, Program};
use crate::data::ast::when::{WhenExpression, WhenPattern};
use crate::data::compilation_dependency::{
    CatalogSignatureParameter, CatalogType, CatalogTypeRef, CatalogVariant, CompilationCatalog,
    TypeSymbol,
};
use crate::data::failures::{
    CallFailure, CompileFailure, CompileFailureKind, CompositeFailure, DeclarationFailure,
    EnumPayloadShape, ResolutionFailure, SemanticArgumentKind, SemanticFailure, SemanticNameRole,
    SemanticTypeDescriptor, SignatureMismatchKind, TypeCheckingFailure, WhenFailure,
};
use crate::data::lexical::SourceSpan;
use crate::data::semantic::SignatureSymbol;
use crate::data::semantic::expressions::{
    SemanticArgument, SemanticCall, SemanticCallTarget, SemanticEnumPayload, SemanticExpression,
    SemanticExpressionKind, SemanticFieldBinding, SemanticFieldValue, SemanticFunctionBody,
    SemanticLiteral, SemanticStatement, SemanticVariantExtraction, SemanticWhen,
    SemanticWhenBranch,
};
use crate::data::semantic::ids::{
    BindingId, FieldId, FunctionId, SignatureBindingId, SignatureId, TypeId, VariantId,
};
use crate::data::semantic::structure::{
    NativeType, SemanticBinding, SemanticField, SemanticFunction, SemanticParameter,
    SemanticProgram, SemanticSignature, SemanticSignatureBinding, SemanticSignatureParameter,
    SemanticType, SemanticVariant,
};

pub type Analyze = for<'source> fn(
    &Program<'source>,
    &CompilationCatalog,
) -> Result<SemanticProgram, CompileFailure>;

pub fn analyze_program<'source>(
    program: &Program<'source>,
    catalog: &CompilationCatalog,
) -> Result<SemanticProgram, CompileFailure> {
    let mut analyzer = Analyzer::new(program, catalog);
    analyzer.analyze()
}

pub const ANALYZE_PROGRAM: Analyze = analyze_program;

// --- Private Analyzer Working State ---

struct Analyzer<'a, 'source> {
    program: &'a Program<'source>,
    catalog: &'a CompilationCatalog,

    types: Vec<SemanticType>,
    type_descriptors: Vec<SemanticTypeDescriptor>,

    // Type lookups (name/symbol -> usize index)
    name_to_type: HashMap<String, usize>,
    shared_type_to_id: HashMap<TypeSymbol, usize>,

    // Struct and Enum field/variant structural metadata for validation
    struct_metadata: HashMap<usize, StructMeta>,
    enum_metadata: HashMap<usize, EnumMeta>,

    // Signatures
    signatures: Vec<SemanticSignature>,
    shared_sig_to_id: HashMap<SignatureSymbol, usize>,
    name_to_signatures: HashMap<String, Vec<usize>>,

    // Functions
    functions: Vec<SemanticFunction>,
    function_headers: Vec<FunctionHeader<'a, 'source>>,
    name_to_function: HashMap<String, usize>,
}

struct StructMeta {
    fields: Vec<FieldMeta>,
    name_to_field_idx: HashMap<String, usize>,
}

struct FieldMeta {
    name: String,
    type_id: usize,
}

struct EnumMeta {
    variants: Vec<VariantMeta>,
    name_to_variant_idx: HashMap<String, usize>,
}

enum VariantMeta {
    Simple {
        name: String,
    },
    Associated {
        name: String,
        type_id: usize,
    },
    Structured {
        name: String,
        fields: Vec<FieldMeta>,
        name_to_field_idx: HashMap<String, usize>,
    },
}

impl VariantMeta {
    fn name(&self) -> &str {
        match self {
            VariantMeta::Simple { name } => name,
            VariantMeta::Associated { name, .. } => name,
            VariantMeta::Structured { name, .. } => name,
        }
    }

    fn payload_shape(&self) -> EnumPayloadShape {
        match self {
            VariantMeta::Simple { .. } => EnumPayloadShape::Simple,
            VariantMeta::Associated { .. } => EnumPayloadShape::Associated,
            VariantMeta::Structured { .. } => EnumPayloadShape::Structured,
        }
    }
}

struct FunctionHeader<'a, 'source> {
    ast: &'a FunctionDefinition<'source>,
    parameters: Vec<FormalParamMeta>,
    result_type: usize,
    satisfaction: Option<(usize, SignatureSymbol)>,
}

enum FormalParamMeta {
    Value {
        name: String,
        type_id: usize,
        span: SourceSpan,
    },
    SignatureDependency {
        name: String,
        signature_id: usize,
        span: SourceSpan,
    },
}

impl<'a, 'source> Analyzer<'a, 'source> {
    fn new(program: &'a Program<'source>, catalog: &'a CompilationCatalog) -> Self {
        let mut analyzer = Self {
            program,
            catalog,
            types: Vec::new(),
            type_descriptors: Vec::new(),
            name_to_type: HashMap::new(),
            shared_type_to_id: HashMap::new(),
            struct_metadata: HashMap::new(),
            enum_metadata: HashMap::new(),
            signatures: Vec::new(),
            shared_sig_to_id: HashMap::new(),
            name_to_signatures: HashMap::new(),
            functions: Vec::new(),
            function_headers: Vec::new(),
            name_to_function: HashMap::new(),
        };
        analyzer.initialize_native_types();
        analyzer
    }

    fn initialize_native_types(&mut self) {
        let natives = [
            (NativeType::Int, "int"),
            (NativeType::Float, "float"),
            (NativeType::Bool, "bool"),
            (NativeType::String, "string"),
            (NativeType::Dynamic, "dynamic"),
            (NativeType::Int8, "int8"),
            (NativeType::Int16, "int16"),
            (NativeType::Int32, "int32"),
            (NativeType::Int64, "int64"),
            (NativeType::Int128, "int128"),
            (NativeType::Uint8, "uint8"),
            (NativeType::Uint16, "uint16"),
            (NativeType::Uint32, "uint32"),
            (NativeType::Uint64, "uint64"),
            (NativeType::Uint128, "uint128"),
            (NativeType::Float32, "float32"),
            (NativeType::Float64, "float64"),
        ];

        for (native_type, name) in natives {
            let id = self.types.len();
            self.types
                .push(SemanticType::Native(clone_native_type(&native_type)));
            self.type_descriptors
                .push(SemanticTypeDescriptor::Native(native_type));
            self.name_to_type.insert(name.to_string(), id);
        }
    }

    fn analyze(&mut self) -> Result<SemanticProgram, CompileFailure> {
        // Step 1: Collect and validate imports
        self.collect_imports()?;

        // Step 2: Register local type shells (structs and enums)
        self.register_local_type_shells()?;

        // Step 3: Populate local type definitions (fields and variants)
        self.populate_local_types()?;

        // Step 4: Validate recursive type cycles
        self.validate_recursive_type_cycles()?;

        // Step 5: Collect and validate function headers
        self.collect_function_headers()?;

        // Step 6: Analyze function bodies
        self.analyze_function_bodies()?;

        // Step 7: Validate function call graph from resolved internal calls (recursion/cycles)
        self.validate_function_call_graph()?;

        // Step 8: Identify public entry function
        let mut entry_function = None;
        for (idx, header) in self.function_headers.iter().enumerate() {
            if matches!(header.ast.visibility, Visibility::Public) {
                entry_function = Some(FunctionId(idx));
                break;
            }
        }
        let entry_function = entry_function.expect("parser guarantees exactly one public function");

        Ok(SemanticProgram {
            types: std::mem::take(&mut self.types),
            signatures: std::mem::take(&mut self.signatures),
            functions: std::mem::take(&mut self.functions),
            entry_function,
        })
    }

    fn is_type_equality_comparable(&self, type_id: usize) -> bool {
        match &self.types[type_id] {
            SemanticType::Native(nt) => !matches!(nt, NativeType::Dynamic),
            SemanticType::Struct { fields } => fields
                .iter()
                .all(|f| self.is_type_equality_comparable(f.type_id.0)),
            SemanticType::Enum { variants } => variants.iter().all(|v| match v {
                SemanticVariant::Simple => true,
                SemanticVariant::Associated { type_id } => {
                    self.is_type_equality_comparable(type_id.0)
                }
                SemanticVariant::Structured { fields } => fields
                    .iter()
                    .all(|f| self.is_type_equality_comparable(f.type_id.0)),
            }),
        }
    }

    fn is_value_type_compatible(&self, actual_id: usize, expected_id: usize) -> bool {
        if actual_id == expected_id {
            return true;
        }
        let expected_type = &self.types[expected_id];
        let actual_type = &self.types[actual_id];
        if let SemanticType::Native(NativeType::Dynamic) = expected_type {
            if let SemanticType::Native(actual_nt) = actual_type {
                return is_numeric_native(actual_nt);
            }
        }
        false
    }

    // --- Step 1: Imports ---

    fn collect_imports(&mut self) -> Result<(), CompileFailure> {
        for import in &self.program.imports {
            let mod_name = import.symbol.qualifier.lexeme;
            let sym_name = import.symbol.name.lexeme;

            let type_sym = TypeSymbol {
                module: mod_name.to_string(),
                name: sym_name.to_string(),
            };

            let sig_sym = SignatureSymbol {
                module: mod_name.to_string(),
                name: sym_name.to_string(),
            };

            let has_type = self.catalog.types.contains_key(&type_sym);
            let has_sig = self.catalog.signatures.contains_key(&sig_sym);

            if !has_type && !has_sig {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Resolution(
                        ResolutionFailure::ImportedSymbolNotFound {
                            module: mod_name.into(),
                            name: sym_name.into(),
                        },
                    )),
                    source_span: SourceSpan {
                        start: import.symbol.qualifier.span.start,
                        end: import.symbol.name.span.end,
                    },
                });
            }

            if has_type {
                let local_name = import
                    .alias
                    .as_ref()
                    .map(|id| id.lexeme)
                    .unwrap_or(sym_name);

                if let Some(alias_id) = &import.alias {
                    if !is_valid_pascal_case(alias_id.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::InvalidNamingConvention {
                                    role: SemanticNameRole::Type,
                                },
                            )),
                            source_span: alias_id.span,
                        });
                    }
                }

                if self.name_to_type.contains_key(local_name) {
                    let span = import
                        .alias
                        .as_ref()
                        .map(|id| id.span)
                        .unwrap_or(import.symbol.name.span);
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                            DeclarationFailure::TypeNameCollision {
                                name: local_name.into(),
                            },
                        )),
                        source_span: span,
                    });
                }

                let type_id = self.materialize_catalog_type(&type_sym);
                self.name_to_type.insert(local_name.to_string(), type_id);
            }

            if has_sig {
                let local_name = import
                    .alias
                    .as_ref()
                    .map(|id| id.lexeme)
                    .unwrap_or(sym_name);

                if let Some(alias_id) = &import.alias {
                    if !is_valid_snake_case(alias_id.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::InvalidNamingConvention {
                                    role: SemanticNameRole::SignatureAlias,
                                },
                            )),
                            source_span: alias_id.span,
                        });
                    }
                }

                let sig_id = self.materialize_catalog_signature(&sig_sym);
                let list = self
                    .name_to_signatures
                    .entry(local_name.to_string())
                    .or_default();
                if !list.contains(&sig_id) {
                    list.push(sig_id);
                }
            }
        }
        Ok(())
    }

    fn materialize_catalog_type(&mut self, symbol: &TypeSymbol) -> usize {
        if let Some(&id) = self.shared_type_to_id.get(symbol) {
            return id;
        }

        let cat_type = self
            .catalog
            .types
            .get(symbol)
            .expect("validated catalog invariant: type exists");

        let type_id = self.types.len();
        self.shared_type_to_id.insert(
            TypeSymbol {
                module: symbol.module.clone(),
                name: symbol.name.clone(),
            },
            type_id,
        );

        match cat_type {
            CatalogType::Struct { fields } => {
                // Reserve slot
                self.types.push(SemanticType::Struct { fields: Vec::new() });
                self.type_descriptors
                    .push(SemanticTypeDescriptor::Shared(TypeSymbol {
                        module: symbol.module.clone(),
                        name: symbol.name.clone(),
                    }));

                let mut sem_fields = Vec::new();
                let mut field_metas = Vec::new();
                let mut name_to_field_idx = HashMap::new();

                for (idx, field) in fields.iter().enumerate() {
                    let field_type_id = self.resolve_catalog_type_ref(&field.type_ref);
                    sem_fields.push(SemanticField {
                        type_id: TypeId(field_type_id),
                    });
                    field_metas.push(FieldMeta {
                        name: field.name.clone(),
                        type_id: field_type_id,
                    });
                    name_to_field_idx.insert(field.name.clone(), idx);
                }

                self.types[type_id] = SemanticType::Struct { fields: sem_fields };
                self.struct_metadata.insert(
                    type_id,
                    StructMeta {
                        fields: field_metas,
                        name_to_field_idx,
                    },
                );
            }
            CatalogType::Enum { variants } => {
                self.types.push(SemanticType::Enum {
                    variants: Vec::new(),
                });
                self.type_descriptors
                    .push(SemanticTypeDescriptor::Shared(TypeSymbol {
                        module: symbol.module.clone(),
                        name: symbol.name.clone(),
                    }));

                let mut sem_variants = Vec::new();
                let mut variant_metas = Vec::new();
                let mut name_to_variant_idx = HashMap::new();

                for (idx, variant) in variants.iter().enumerate() {
                    match variant {
                        CatalogVariant::Simple { name } => {
                            sem_variants.push(SemanticVariant::Simple);
                            variant_metas.push(VariantMeta::Simple { name: name.clone() });
                            name_to_variant_idx.insert(name.clone(), idx);
                        }
                        CatalogVariant::Associated { name, type_ref } => {
                            let payload_type_id = self.resolve_catalog_type_ref(type_ref);
                            sem_variants.push(SemanticVariant::Associated {
                                type_id: TypeId(payload_type_id),
                            });
                            variant_metas.push(VariantMeta::Associated {
                                name: name.clone(),
                                type_id: payload_type_id,
                            });
                            name_to_variant_idx.insert(name.clone(), idx);
                        }
                        CatalogVariant::Structured { name, fields } => {
                            let mut sem_fields = Vec::new();
                            let mut field_metas = Vec::new();
                            let mut f_name_to_idx = HashMap::new();

                            for (f_idx, field) in fields.iter().enumerate() {
                                let f_type_id = self.resolve_catalog_type_ref(&field.type_ref);
                                sem_fields.push(SemanticField {
                                    type_id: TypeId(f_type_id),
                                });
                                field_metas.push(FieldMeta {
                                    name: field.name.clone(),
                                    type_id: f_type_id,
                                });
                                f_name_to_idx.insert(field.name.clone(), f_idx);
                            }

                            sem_variants.push(SemanticVariant::Structured { fields: sem_fields });
                            variant_metas.push(VariantMeta::Structured {
                                name: name.clone(),
                                fields: field_metas,
                                name_to_field_idx: f_name_to_idx,
                            });
                            name_to_variant_idx.insert(name.clone(), idx);
                        }
                    }
                }

                self.types[type_id] = SemanticType::Enum {
                    variants: sem_variants,
                };
                self.enum_metadata.insert(
                    type_id,
                    EnumMeta {
                        variants: variant_metas,
                        name_to_variant_idx,
                    },
                );
            }
        }

        type_id
    }

    fn resolve_catalog_type_ref(&mut self, type_ref: &CatalogTypeRef) -> usize {
        match type_ref {
            CatalogTypeRef::Int => self.name_to_type["int"],
            CatalogTypeRef::Float => self.name_to_type["float"],
            CatalogTypeRef::Bool => self.name_to_type["bool"],
            CatalogTypeRef::String => self.name_to_type["string"],
            CatalogTypeRef::Dynamic => self.name_to_type["dynamic"],
            CatalogTypeRef::Int8 => self.name_to_type["int8"],
            CatalogTypeRef::Int16 => self.name_to_type["int16"],
            CatalogTypeRef::Int32 => self.name_to_type["int32"],
            CatalogTypeRef::Int64 => self.name_to_type["int64"],
            CatalogTypeRef::Int128 => self.name_to_type["int128"],
            CatalogTypeRef::Uint8 => self.name_to_type["uint8"],
            CatalogTypeRef::Uint16 => self.name_to_type["uint16"],
            CatalogTypeRef::Uint32 => self.name_to_type["uint32"],
            CatalogTypeRef::Uint64 => self.name_to_type["uint64"],
            CatalogTypeRef::Uint128 => self.name_to_type["uint128"],
            CatalogTypeRef::Float32 => self.name_to_type["float32"],
            CatalogTypeRef::Float64 => self.name_to_type["float64"],
            CatalogTypeRef::Shared(type_sym) => self.materialize_catalog_type(type_sym),
        }
    }

    fn materialize_catalog_signature(&mut self, symbol: &SignatureSymbol) -> usize {
        if let Some(&id) = self.shared_sig_to_id.get(symbol) {
            return id;
        }

        let cat_sig = self
            .catalog
            .signatures
            .get(symbol)
            .expect("validated catalog invariant: signature exists");

        let sig_id = self.signatures.len();
        self.shared_sig_to_id.insert(
            SignatureSymbol {
                module: symbol.module.clone(),
                name: symbol.name.clone(),
            },
            sig_id,
        );

        // Reserve identity slot before resolving transitive dependencies
        self.signatures.push(SemanticSignature {
            symbol: SignatureSymbol {
                module: symbol.module.clone(),
                name: symbol.name.clone(),
            },
            parameters: Vec::new(),
            result_type: TypeId(0),
        });

        let mut parameters = Vec::new();
        for param in &cat_sig.parameters {
            match param {
                CatalogSignatureParameter::Value(type_ref) => {
                    let type_id = self.resolve_catalog_type_ref(type_ref);
                    parameters.push(SemanticSignatureParameter::Value(TypeId(type_id)));
                }
                CatalogSignatureParameter::SignatureDependency(dep_sym) => {
                    let dep_id = self.materialize_catalog_signature(dep_sym);
                    parameters.push(SemanticSignatureParameter::SignatureDependency(
                        SignatureId(dep_id),
                    ));
                }
            }
        }

        let result_type = self.resolve_catalog_type_ref(&cat_sig.result_type);

        self.signatures[sig_id] = SemanticSignature {
            symbol: SignatureSymbol {
                module: symbol.module.clone(),
                name: symbol.name.clone(),
            },
            parameters,
            result_type: TypeId(result_type),
        };

        sig_id
    }

    // --- Step 2: Register Local Type Shells ---

    fn register_local_type_shells(&mut self) -> Result<(), CompileFailure> {
        for decl in &self.program.declarations {
            match decl {
                Declaration::Struct(st) => {
                    if !is_valid_pascal_case(st.name.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::InvalidNamingConvention {
                                    role: SemanticNameRole::Type,
                                },
                            )),
                            source_span: st.name.span,
                        });
                    }

                    if self.name_to_type.contains_key(st.name.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::TypeNameCollision {
                                    name: st.name.lexeme.into(),
                                },
                            )),
                            source_span: st.name.span,
                        });
                    }

                    let type_id = self.types.len();
                    self.types.push(SemanticType::Struct { fields: Vec::new() });
                    self.type_descriptors
                        .push(SemanticTypeDescriptor::Local(st.name.lexeme.into()));
                    self.name_to_type
                        .insert(st.name.lexeme.to_string(), type_id);
                }
                Declaration::Enum(en) => {
                    if !is_valid_pascal_case(en.name.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::InvalidNamingConvention {
                                    role: SemanticNameRole::Type,
                                },
                            )),
                            source_span: en.name.span,
                        });
                    }

                    if self.name_to_type.contains_key(en.name.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::TypeNameCollision {
                                    name: en.name.lexeme.into(),
                                },
                            )),
                            source_span: en.name.span,
                        });
                    }

                    let type_id = self.types.len();
                    self.types.push(SemanticType::Enum {
                        variants: Vec::new(),
                    });
                    self.type_descriptors
                        .push(SemanticTypeDescriptor::Local(en.name.lexeme.into()));
                    self.name_to_type
                        .insert(en.name.lexeme.to_string(), type_id);
                }
                Declaration::Function(_) => {}
            }
        }
        Ok(())
    }

    // --- Step 3: Populate Local Types ---

    fn populate_local_types(&mut self) -> Result<(), CompileFailure> {
        for decl in &self.program.declarations {
            match decl {
                Declaration::Struct(st) => {
                    let type_id = self.name_to_type[st.name.lexeme];
                    let mut sem_fields = Vec::new();
                    let mut field_metas = Vec::new();
                    let mut name_to_field_idx = HashMap::new();

                    for field in &st.fields {
                        if !is_valid_snake_case(field.name.lexeme) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                    DeclarationFailure::InvalidNamingConvention {
                                        role: SemanticNameRole::Field,
                                    },
                                )),
                                source_span: field.name.span,
                            });
                        }

                        if name_to_field_idx.contains_key(field.name.lexeme) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                    DeclarationFailure::DuplicateField {
                                        name: field.name.lexeme.into(),
                                    },
                                )),
                                source_span: field.name.span,
                            });
                        }

                        let field_type_id = self.resolve_type_identifier(&field.type_name)?;
                        let idx = sem_fields.len();
                        sem_fields.push(SemanticField {
                            type_id: TypeId(field_type_id),
                        });
                        field_metas.push(FieldMeta {
                            name: field.name.lexeme.to_string(),
                            type_id: field_type_id,
                        });
                        name_to_field_idx.insert(field.name.lexeme.to_string(), idx);
                    }

                    self.types[type_id] = SemanticType::Struct { fields: sem_fields };
                    self.struct_metadata.insert(
                        type_id,
                        StructMeta {
                            fields: field_metas,
                            name_to_field_idx,
                        },
                    );
                }
                Declaration::Enum(en) => {
                    let type_id = self.name_to_type[en.name.lexeme];
                    let mut sem_variants = Vec::new();
                    let mut variant_metas = Vec::new();
                    let mut name_to_variant_idx = HashMap::new();

                    for variant in &en.variants {
                        let (var_name, var_span) = match variant {
                            EnumVariant::Simple { name } => (name.lexeme, name.span),
                            EnumVariant::Associated { name, .. } => (name.lexeme, name.span),
                            EnumVariant::Structured { name, .. } => (name.lexeme, name.span),
                        };

                        if !is_valid_pascal_case(var_name) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                    DeclarationFailure::InvalidNamingConvention {
                                        role: SemanticNameRole::Variant,
                                    },
                                )),
                                source_span: var_span,
                            });
                        }

                        if name_to_variant_idx.contains_key(var_name) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                    DeclarationFailure::DuplicateVariant {
                                        name: var_name.into(),
                                    },
                                )),
                                source_span: var_span,
                            });
                        }

                        let v_idx = sem_variants.len();
                        name_to_variant_idx.insert(var_name.to_string(), v_idx);

                        match variant {
                            EnumVariant::Simple { .. } => {
                                sem_variants.push(SemanticVariant::Simple);
                                variant_metas.push(VariantMeta::Simple {
                                    name: var_name.to_string(),
                                });
                            }
                            EnumVariant::Associated { type_name, .. } => {
                                let payload_type_id = self.resolve_type_identifier(type_name)?;
                                sem_variants.push(SemanticVariant::Associated {
                                    type_id: TypeId(payload_type_id),
                                });
                                variant_metas.push(VariantMeta::Associated {
                                    name: var_name.to_string(),
                                    type_id: payload_type_id,
                                });
                            }
                            EnumVariant::Structured { fields, .. } => {
                                let mut sem_fields = Vec::new();
                                let mut field_metas = Vec::new();
                                let mut f_name_to_idx = HashMap::new();

                                for field in fields {
                                    if !is_valid_snake_case(field.name.lexeme) {
                                        return Err(CompileFailure {
                                            kind: CompileFailureKind::Semantic(
                                                SemanticFailure::Declaration(
                                                    DeclarationFailure::InvalidNamingConvention {
                                                        role: SemanticNameRole::Field,
                                                    },
                                                ),
                                            ),
                                            source_span: field.name.span,
                                        });
                                    }

                                    if f_name_to_idx.contains_key(field.name.lexeme) {
                                        return Err(CompileFailure {
                                            kind: CompileFailureKind::Semantic(
                                                SemanticFailure::Declaration(
                                                    DeclarationFailure::DuplicateField {
                                                        name: field.name.lexeme.into(),
                                                    },
                                                ),
                                            ),
                                            source_span: field.name.span,
                                        });
                                    }

                                    let f_type_id =
                                        self.resolve_type_identifier(&field.type_name)?;
                                    let f_idx = sem_fields.len();
                                    sem_fields.push(SemanticField {
                                        type_id: TypeId(f_type_id),
                                    });
                                    field_metas.push(FieldMeta {
                                        name: field.name.lexeme.to_string(),
                                        type_id: f_type_id,
                                    });
                                    f_name_to_idx.insert(field.name.lexeme.to_string(), f_idx);
                                }

                                sem_variants
                                    .push(SemanticVariant::Structured { fields: sem_fields });
                                variant_metas.push(VariantMeta::Structured {
                                    name: var_name.to_string(),
                                    fields: field_metas,
                                    name_to_field_idx: f_name_to_idx,
                                });
                            }
                        }
                    }

                    self.types[type_id] = SemanticType::Enum {
                        variants: sem_variants,
                    };
                    self.enum_metadata.insert(
                        type_id,
                        EnumMeta {
                            variants: variant_metas,
                            name_to_variant_idx,
                        },
                    );
                }
                Declaration::Function(_) => {}
            }
        }
        Ok(())
    }

    fn resolve_type_identifier(&self, id: &Identifier<'source>) -> Result<usize, CompileFailure> {
        if let Some(&type_id) = self.name_to_type.get(id.lexeme) {
            Ok(type_id)
        } else {
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Resolution(
                    ResolutionFailure::UnknownType {
                        name: id.lexeme.into(),
                    },
                )),
                source_span: id.span,
            })
        }
    }

    // --- Step 4: Recursive Type Cycles ---

    fn validate_recursive_type_cycles(&self) -> Result<(), CompileFailure> {
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

        for decl in &self.program.declarations {
            match decl {
                Declaration::Struct(st) => {
                    let type_id = self.name_to_type[st.name.lexeme];
                    let mut neighbors = Vec::new();
                    if let Some(meta) = self.struct_metadata.get(&type_id) {
                        for f in &meta.fields {
                            if matches!(
                                self.types[f.type_id],
                                SemanticType::Struct { .. } | SemanticType::Enum { .. }
                            ) {
                                neighbors.push(f.type_id);
                            }
                        }
                    }
                    adj.insert(type_id, neighbors);
                }
                Declaration::Enum(en) => {
                    let type_id = self.name_to_type[en.name.lexeme];
                    let mut neighbors = Vec::new();
                    if let Some(meta) = self.enum_metadata.get(&type_id) {
                        for v in &meta.variants {
                            match v {
                                VariantMeta::Simple { .. } => {}
                                VariantMeta::Associated { type_id: tid, .. } => {
                                    if matches!(
                                        self.types[*tid],
                                        SemanticType::Struct { .. } | SemanticType::Enum { .. }
                                    ) {
                                        neighbors.push(*tid);
                                    }
                                }
                                VariantMeta::Structured { fields, .. } => {
                                    for f in fields {
                                        if matches!(
                                            self.types[f.type_id],
                                            SemanticType::Struct { .. } | SemanticType::Enum { .. }
                                        ) {
                                            neighbors.push(f.type_id);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    adj.insert(type_id, neighbors);
                }
                Declaration::Function(_) => {}
            }
        }

        // Cycle detection using 3-color DFS
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Color {
            White,
            Gray,
            Black,
        }

        let mut colors: HashMap<usize, Color> = HashMap::new();
        for &tid in adj.keys() {
            colors.insert(tid, Color::White);
        }

        fn dfs(
            node: usize,
            adj: &HashMap<usize, Vec<usize>>,
            colors: &mut HashMap<usize, Color>,
        ) -> bool {
            colors.insert(node, Color::Gray);
            if let Some(neighbors) = adj.get(&node) {
                for &neighbor in neighbors {
                    match colors.get(&neighbor) {
                        Some(Color::Gray) => return true,
                        Some(Color::White) => {
                            if dfs(neighbor, adj, colors) {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }
            }
            colors.insert(node, Color::Black);
            false
        }

        for decl in &self.program.declarations {
            let (tid, span) = match decl {
                Declaration::Struct(st) => (self.name_to_type[st.name.lexeme], st.name.span),
                Declaration::Enum(en) => (self.name_to_type[en.name.lexeme], en.name.span),
                Declaration::Function(_) => continue,
            };

            if colors.get(&tid) == Some(&Color::White) {
                if dfs(tid, &adj, &mut colors) {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                            DeclarationFailure::RecursiveTypeCycle,
                        )),
                        source_span: span,
                    });
                }
            }
        }

        Ok(())
    }

    // --- Step 5: Function Headers ---

    fn collect_function_headers(&mut self) -> Result<(), CompileFailure> {
        for decl in &self.program.declarations {
            if let Declaration::Function(fn_def) = decl {
                let fn_name = fn_def.name.lexeme;

                if !is_valid_snake_case(fn_name) {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                            DeclarationFailure::InvalidNamingConvention {
                                role: SemanticNameRole::Function,
                            },
                        )),
                        source_span: fn_def.name.span,
                    });
                }

                if self.name_to_function.contains_key(fn_name) {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                            DeclarationFailure::DuplicateFunction {
                                name: fn_name.into(),
                            },
                        )),
                        source_span: fn_def.name.span,
                    });
                }

                let func_id = self.function_headers.len();
                self.name_to_function.insert(fn_name.to_string(), func_id);

                let mut params = Vec::new();
                let mut param_names = HashSet::new();

                for param in &fn_def.parameters {
                    match param {
                        Parameter::Value(binding) => {
                            if !is_valid_snake_case(binding.name.lexeme) {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(
                                        SemanticFailure::Declaration(
                                            DeclarationFailure::InvalidNamingConvention {
                                                role: SemanticNameRole::Binding,
                                            },
                                        ),
                                    ),
                                    source_span: binding.name.span,
                                });
                            }

                            if param_names.contains(binding.name.lexeme) {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(
                                        SemanticFailure::Declaration(
                                            DeclarationFailure::BindingNameCollision {
                                                name: binding.name.lexeme.into(),
                                            },
                                        ),
                                    ),
                                    source_span: binding.name.span,
                                });
                            }
                            param_names.insert(binding.name.lexeme);

                            let param_type_id = self.resolve_type_identifier(&binding.type_name)?;
                            params.push(FormalParamMeta::Value {
                                name: binding.name.lexeme.to_string(),
                                type_id: param_type_id,
                                span: binding.name.span,
                            });
                        }
                        Parameter::SignatureDependency { signature, name } => {
                            if !is_valid_snake_case(name.lexeme) {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(
                                        SemanticFailure::Declaration(
                                            DeclarationFailure::InvalidNamingConvention {
                                                role: SemanticNameRole::SignatureDependency,
                                            },
                                        ),
                                    ),
                                    source_span: name.span,
                                });
                            }

                            if param_names.contains(name.lexeme) {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(
                                        SemanticFailure::Declaration(
                                            DeclarationFailure::BindingNameCollision {
                                                name: name.lexeme.into(),
                                            },
                                        ),
                                    ),
                                    source_span: name.span,
                                });
                            }
                            param_names.insert(name.lexeme);

                            let sig_sym = SignatureSymbol {
                                module: signature.qualifier.lexeme.to_string(),
                                name: signature.name.lexeme.to_string(),
                            };

                            if !self.catalog.signatures.contains_key(&sig_sym) {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(
                                        SemanticFailure::Resolution(
                                            ResolutionFailure::UnknownSignature(sig_sym),
                                        ),
                                    ),
                                    source_span: SourceSpan {
                                        start: signature.qualifier.span.start,
                                        end: signature.name.span.end,
                                    },
                                });
                            }

                            let sig_id = self.materialize_catalog_signature(&sig_sym);

                            params.push(FormalParamMeta::SignatureDependency {
                                name: name.lexeme.to_string(),
                                signature_id: sig_id,
                                span: name.span,
                            });
                        }
                    }
                }

                let result_type = self.resolve_type_identifier(&fn_def.result_type)?;

                let satisfaction = if let Some(sat_name) = &fn_def.satisfaction {
                    let sig_sym = SignatureSymbol {
                        module: sat_name.qualifier.lexeme.to_string(),
                        name: sat_name.name.lexeme.to_string(),
                    };

                    if !self.catalog.signatures.contains_key(&sig_sym) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Resolution(
                                ResolutionFailure::UnknownSignature(sig_sym),
                            )),
                            source_span: SourceSpan {
                                start: sat_name.qualifier.span.start,
                                end: sat_name.name.span.end,
                            },
                        });
                    }

                    let sig_id = self.materialize_catalog_signature(&sig_sym);

                    self.validate_signature_satisfaction(
                        fn_def,
                        &params,
                        result_type,
                        sig_id,
                        &sig_sym,
                        sat_name,
                    )?;
                    Some((sig_id, sig_sym))
                } else {
                    None
                };

                self.function_headers.push(FunctionHeader {
                    ast: fn_def,
                    parameters: params,
                    result_type,
                    satisfaction,
                });
            }
        }
        Ok(())
    }

    fn validate_signature_satisfaction(
        &self,
        fn_def: &FunctionDefinition<'source>,
        params: &[FormalParamMeta],
        result_type: usize,
        sig_id: usize,
        sig_sym: &SignatureSymbol,
        sat_name: &QualifiedName<'source>,
    ) -> Result<(), CompileFailure> {
        let sat_span = SourceSpan {
            start: sat_name.qualifier.span.start,
            end: sat_name.name.span.end,
        };

        // 1. Function Name
        if fn_def.name.lexeme != sig_sym.name {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                    signature: SignatureSymbol {
                        module: sig_sym.module.clone(),
                        name: sig_sym.name.clone(),
                    },
                    mismatch: SignatureMismatchKind::FunctionName,
                }),
                source_span: fn_def.name.span,
            });
        }

        let sig = &self.signatures[sig_id];

        // 2. Parameter Count
        if params.len() != sig.parameters.len() {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                    signature: SignatureSymbol {
                        module: sig_sym.module.clone(),
                        name: sig_sym.name.clone(),
                    },
                    mismatch: SignatureMismatchKind::ParameterCount {
                        expected: sig.parameters.len(),
                        actual: params.len(),
                    },
                }),
                source_span: sat_span,
            });
        }

        // 3. Parameters in positional order
        for (i, (actual_param, expected_param)) in params.iter().zip(&sig.parameters).enumerate() {
            match (actual_param, expected_param) {
                (
                    FormalParamMeta::Value { type_id, span, .. },
                    SemanticSignatureParameter::Value(expected_tid),
                ) => {
                    if *type_id != expected_tid.0 {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(
                                SemanticFailure::SignatureMismatch {
                                    signature: SignatureSymbol {
                                        module: sig_sym.module.clone(),
                                        name: sig_sym.name.clone(),
                                    },
                                    mismatch: SignatureMismatchKind::ValueParameterType {
                                        position: i,
                                        expected: clone_type_descriptor(
                                            &self.type_descriptors[expected_tid.0],
                                        ),
                                        actual: clone_type_descriptor(
                                            &self.type_descriptors[*type_id],
                                        ),
                                    },
                                },
                            ),
                            source_span: *span,
                        });
                    }
                }
                (
                    FormalParamMeta::SignatureDependency {
                        signature_id, span, ..
                    },
                    SemanticSignatureParameter::SignatureDependency(expected_sid),
                ) => {
                    if *signature_id != expected_sid.0 {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(
                                SemanticFailure::SignatureMismatch {
                                    signature: SignatureSymbol {
                                        module: sig_sym.module.clone(),
                                        name: sig_sym.name.clone(),
                                    },
                                    mismatch: SignatureMismatchKind::SignatureDependency {
                                        position: i,
                                        expected: clone_signature_symbol(
                                            &self.signatures[expected_sid.0].symbol,
                                        ),
                                        actual: clone_signature_symbol(
                                            &self.signatures[*signature_id].symbol,
                                        ),
                                    },
                                },
                            ),
                            source_span: *span,
                        });
                    }
                }
                (
                    FormalParamMeta::Value { span, .. },
                    SemanticSignatureParameter::SignatureDependency(_),
                ) => {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                            signature: SignatureSymbol {
                                module: sig_sym.module.clone(),
                                name: sig_sym.name.clone(),
                            },
                            mismatch: SignatureMismatchKind::ParameterKind {
                                position: i,
                                expected: SemanticArgumentKind::SignatureDependency,
                                actual: SemanticArgumentKind::Value,
                            },
                        }),
                        source_span: *span,
                    });
                }
                (
                    FormalParamMeta::SignatureDependency { span, .. },
                    SemanticSignatureParameter::Value(_),
                ) => {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                            signature: SignatureSymbol {
                                module: sig_sym.module.clone(),
                                name: sig_sym.name.clone(),
                            },
                            mismatch: SignatureMismatchKind::ParameterKind {
                                position: i,
                                expected: SemanticArgumentKind::Value,
                                actual: SemanticArgumentKind::SignatureDependency,
                            },
                        }),
                        source_span: *span,
                    });
                }
            }
        }

        // 4. Result Type
        if result_type != sig.result_type.0 {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                    signature: SignatureSymbol {
                        module: sig_sym.module.clone(),
                        name: sig_sym.name.clone(),
                    },
                    mismatch: SignatureMismatchKind::ResultType {
                        expected: clone_type_descriptor(&self.type_descriptors[sig.result_type.0]),
                        actual: clone_type_descriptor(&self.type_descriptors[result_type]),
                    },
                }),
                source_span: fn_def.result_type.span,
            });
        }

        Ok(())
    }

    // --- Step 6: Analyze Function Bodies ---

    fn analyze_function_bodies(&mut self) -> Result<(), CompileFailure> {
        let headers_len = self.function_headers.len();
        for i in 0..headers_len {
            let func = {
                let header = &self.function_headers[i];
                let mut fn_analyzer = FunctionAnalyzer::new(self, header);
                fn_analyzer.analyze_body()?
            };
            self.functions.push(func);
        }
        Ok(())
    }

    // --- Step 7: Validate Function Call Graph (Recursion/Cycles) ---

    fn validate_function_call_graph(&self) -> Result<(), CompileFailure> {
        let mut adj: HashMap<usize, Vec<(usize, SourceSpan)>> = HashMap::new();

        fn collect_internal_calls(expr: &SemanticExpression, calls: &mut Vec<(usize, SourceSpan)>) {
            match &expr.kind {
                SemanticExpressionKind::Call(call) => {
                    if let SemanticCallTarget::Internal(fid) = &call.target {
                        calls.push((fid.0, expr.span));
                    }
                    for arg in &call.arguments {
                        if let SemanticArgument::Value(val) = arg {
                            collect_internal_calls(val, calls);
                        }
                    }
                }
                SemanticExpressionKind::Conversion { operand } => {
                    collect_internal_calls(operand, calls);
                }
                SemanticExpressionKind::Unary { operand, .. } => {
                    collect_internal_calls(operand, calls);
                }
                SemanticExpressionKind::Binary { left, right, .. } => {
                    collect_internal_calls(left, calls);
                    collect_internal_calls(right, calls);
                }
                SemanticExpressionKind::FieldAccess { receiver, .. } => {
                    collect_internal_calls(receiver, calls);
                }
                SemanticExpressionKind::StructConstruction { fields } => {
                    for f in fields {
                        collect_internal_calls(&f.value, calls);
                    }
                }
                SemanticExpressionKind::EnumConstruction { payload, .. } => match payload {
                    SemanticEnumPayload::Simple => {}
                    SemanticEnumPayload::Associated { value } => {
                        collect_internal_calls(value, calls);
                    }
                    SemanticEnumPayload::Structured { fields } => {
                        for f in fields {
                            collect_internal_calls(&f.value, calls);
                        }
                    }
                },
                SemanticExpressionKind::When(when) => {
                    collect_internal_calls(&when.subject, calls);
                    for branch in &when.branches {
                        collect_internal_calls(&branch.result, calls);
                    }
                }
                SemanticExpressionKind::Literal(_) | SemanticExpressionKind::Binding(_) => {}
            }
        }

        for (caller_id, func) in self.functions.iter().enumerate() {
            let mut calls = Vec::new();
            for stmt in &func.body.statements {
                match stmt {
                    SemanticStatement::Bind { value, .. } => {
                        collect_internal_calls(value, &mut calls);
                    }
                    SemanticStatement::Operation(op) => {
                        collect_internal_calls(op, &mut calls);
                    }
                }
            }
            collect_internal_calls(&func.body.result, &mut calls);
            adj.insert(caller_id, calls);
        }

        // Cycle check using 3-color DFS
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Color {
            White,
            Gray,
            Black,
        }

        let mut colors: HashMap<usize, Color> = HashMap::new();
        for i in 0..self.functions.len() {
            colors.insert(i, Color::White);
        }

        fn dfs_calls(
            node: usize,
            adj: &HashMap<usize, Vec<(usize, SourceSpan)>>,
            colors: &mut HashMap<usize, Color>,
        ) -> Option<SourceSpan> {
            colors.insert(node, Color::Gray);
            if let Some(calls) = adj.get(&node) {
                for (target, span) in calls {
                    match colors.get(target) {
                        Some(Color::Gray) => return Some(*span),
                        Some(Color::White) => {
                            if let Some(cycle_span) = dfs_calls(*target, adj, colors) {
                                return Some(cycle_span);
                            }
                        }
                        _ => {}
                    }
                }
            }
            colors.insert(node, Color::Black);
            None
        }

        for i in 0..self.functions.len() {
            if colors.get(&i) == Some(&Color::White) {
                if let Some(cycle_span) = dfs_calls(i, &adj, &mut colors) {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                            CallFailure::FunctionCallCycle,
                        )),
                        source_span: cycle_span,
                    });
                }
            }
        }

        Ok(())
    }
}

// --- Function Analyzer ---

struct FunctionAnalyzer<'analyzer, 'a, 'source> {
    analyzer: &'analyzer Analyzer<'a, 'source>,
    header: &'analyzer FunctionHeader<'a, 'source>,

    parameters: Vec<SemanticParameter>,
    bindings: Vec<SemanticBinding>,
    signature_bindings: Vec<SemanticSignatureBinding>,

    // Scope tables (name -> usize index)
    name_to_binding: HashMap<String, usize>,
    name_to_sig_binding: HashMap<String, usize>,
}

impl<'analyzer, 'a, 'source> FunctionAnalyzer<'analyzer, 'a, 'source> {
    fn new(
        analyzer: &'analyzer Analyzer<'a, 'source>,
        header: &'analyzer FunctionHeader<'a, 'source>,
    ) -> Self {
        let mut fa = Self {
            analyzer,
            header,
            parameters: Vec::new(),
            bindings: Vec::new(),
            signature_bindings: Vec::new(),
            name_to_binding: HashMap::new(),
            name_to_sig_binding: HashMap::new(),
        };

        for param in &header.parameters {
            match param {
                FormalParamMeta::Value { name, type_id, .. } => {
                    let bid = fa.bindings.len();
                    fa.bindings.push(SemanticBinding {
                        type_id: TypeId(*type_id),
                    });
                    fa.parameters.push(SemanticParameter::Value(BindingId(bid)));
                    fa.name_to_binding.insert(name.clone(), bid);
                }
                FormalParamMeta::SignatureDependency {
                    name, signature_id, ..
                } => {
                    let sbid = fa.signature_bindings.len();
                    fa.signature_bindings.push(SemanticSignatureBinding {
                        signature: SignatureId(*signature_id),
                    });
                    fa.parameters
                        .push(SemanticParameter::SignatureDependency(SignatureBindingId(
                            sbid,
                        )));
                    fa.name_to_sig_binding.insert(name.clone(), sbid);
                }
            }
        }

        fa
    }

    fn analyze_body(&mut self) -> Result<SemanticFunction, CompileFailure> {
        let mut statements = Vec::new();

        for stmt in &self.header.ast.body.statements {
            match stmt {
                BodyStatement::Let(let_bind) => {
                    let bind_name = let_bind.binding.name.lexeme;

                    if !is_valid_snake_case(bind_name) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::InvalidNamingConvention {
                                    role: SemanticNameRole::Binding,
                                },
                            )),
                            source_span: let_bind.binding.name.span,
                        });
                    }

                    if self.name_to_binding.contains_key(bind_name)
                        || self.name_to_sig_binding.contains_key(bind_name)
                    {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::BindingNameCollision {
                                    name: bind_name.into(),
                                },
                            )),
                            source_span: let_bind.binding.name.span,
                        });
                    }

                    let expected_type = self
                        .analyzer
                        .resolve_type_identifier(&let_bind.binding.type_name)?;

                    let val_expr = self.analyze_expression(&let_bind.value, Some(expected_type))?;

                    if !self
                        .analyzer
                        .is_value_type_compatible(val_expr.type_id.0, expected_type)
                    {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                                TypeCheckingFailure::BindingInitialization {
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[expected_type],
                                    ),
                                    actual: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[val_expr.type_id.0],
                                    ),
                                },
                            )),
                            source_span: val_expr.span,
                        });
                    }

                    let bid = self.bindings.len();
                    self.bindings.push(SemanticBinding {
                        type_id: TypeId(expected_type),
                    });
                    self.name_to_binding.insert(bind_name.to_string(), bid);

                    statements.push(SemanticStatement::Bind {
                        binding: BindingId(bid),
                        value: val_expr,
                    });
                }
                BodyStatement::Operation(op_stmt) => {
                    let expr = match op_stmt {
                        OperationStatement::FunctionCall(call) => {
                            self.analyze_function_call(call, call.callee.span, None)?
                        }
                        OperationStatement::Pipeline(pipe) => self.analyze_pipeline(pipe, None)?,
                    };
                    statements.push(SemanticStatement::Operation(expr));
                }
            }
        }

        let result =
            self.analyze_expression(&self.header.ast.body.result, Some(self.header.result_type))?;

        if !self
            .analyzer
            .is_value_type_compatible(result.type_id.0, self.header.result_type)
        {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::FunctionResult {
                        expected: clone_type_descriptor(
                            &self.analyzer.type_descriptors[self.header.result_type],
                        ),
                        actual: clone_type_descriptor(
                            &self.analyzer.type_descriptors[result.type_id.0],
                        ),
                    },
                )),
                source_span: result.span,
            });
        }

        Ok(SemanticFunction {
            parameters: std::mem::take(&mut self.parameters),
            bindings: std::mem::take(&mut self.bindings),
            signature_bindings: std::mem::take(&mut self.signature_bindings),
            result_type: TypeId(self.header.result_type),
            satisfaction: self
                .header
                .satisfaction
                .as_ref()
                .map(|(sid, _)| SignatureId(*sid)),
            body: SemanticFunctionBody { statements, result },
        })
    }

    // --- Expression Analysis ---

    fn analyze_expression(
        &mut self,
        expr: &Expression<'source>,
        expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        match &expr.kind {
            ExpressionKind::Literal { kind, lexeme } => {
                self.analyze_literal(kind, lexeme, expr.span, expected_type)
            }
            ExpressionKind::Identifier(id) => self.analyze_identifier(id),
            ExpressionKind::Unary { operator, operand } => {
                self.analyze_unary(operator, operand, expr.span, expected_type)
            }
            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => self.analyze_binary(left, operator, right, expr.span, expected_type),
            ExpressionKind::FieldAccess { receiver, field } => {
                self.analyze_field_access(receiver, field, expr.span)
            }
            ExpressionKind::StructConstruction { type_name, fields } => {
                self.analyze_struct_construction(type_name, fields, expr.span)
            }
            ExpressionKind::EnumConstruction(ec) => self.analyze_enum_construction(ec, expr.span),
            ExpressionKind::FunctionCall(call) => {
                self.analyze_function_call(call, expr.span, expected_type)
            }
            ExpressionKind::Pipeline(pipe) => self.analyze_pipeline(pipe, expected_type),
            ExpressionKind::When(when) => self.analyze_when(when, expr.span, expected_type),
        }
    }

    fn analyze_literal(
        &self,
        kind: &LiteralKind,
        lexeme: &'source str,
        span: SourceSpan,
        expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        match kind {
            LiteralKind::Integer => {
                let (type_id, canonical_str) =
                    self.resolve_integer_literal(lexeme, span, expected_type, false)?;
                Ok(SemanticExpression {
                    type_id: TypeId(type_id),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(canonical_str)),
                    span,
                })
            }
            LiteralKind::Floating => {
                let target_type = if let Some(exp_tid) = expected_type {
                    if matches!(
                        self.analyzer.types[exp_tid],
                        SemanticType::Native(
                            NativeType::Float
                                | NativeType::Float32
                                | NativeType::Float64
                                | NativeType::Dynamic
                        )
                    ) {
                        exp_tid
                    } else {
                        self.analyzer.name_to_type["float"]
                    }
                } else {
                    self.analyzer.name_to_type["float"]
                };

                let target_native = match &self.analyzer.types[target_type] {
                    SemanticType::Native(nt) => nt,
                    _ => &NativeType::Float,
                };

                let val: f64 = match target_native {
                    NativeType::Float32 => {
                        let f: f32 = lexeme.parse().map_err(|_| CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                                TypeCheckingFailure::NumericLiteralNotRepresentable {
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[target_type],
                                    ),
                                },
                            )),
                            source_span: span,
                        })?;
                        if !f.is_finite() {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                                    TypeCheckingFailure::NumericLiteralNotRepresentable {
                                        expected: clone_type_descriptor(
                                            &self.analyzer.type_descriptors[target_type],
                                        ),
                                    },
                                )),
                                source_span: span,
                            });
                        }
                        f as f64
                    }
                    _ => {
                        let f: f64 = lexeme.parse().map_err(|_| CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                                TypeCheckingFailure::NumericLiteralNotRepresentable {
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[target_type],
                                    ),
                                },
                            )),
                            source_span: span,
                        })?;
                        if !f.is_finite() {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                                    TypeCheckingFailure::NumericLiteralNotRepresentable {
                                        expected: clone_type_descriptor(
                                            &self.analyzer.type_descriptors[target_type],
                                        ),
                                    },
                                )),
                                source_span: span,
                            });
                        }
                        f
                    }
                };

                Ok(SemanticExpression {
                    type_id: TypeId(target_type),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Floating(val)),
                    span,
                })
            }
            LiteralKind::Boolean => {
                let val = lexeme == "true";
                let type_id = self.analyzer.name_to_type["bool"];
                Ok(SemanticExpression {
                    type_id: TypeId(type_id),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Boolean(val)),
                    span,
                })
            }
            LiteralKind::String => {
                let decoded = decode_string_literal(lexeme);
                let type_id = self.analyzer.name_to_type["string"];
                Ok(SemanticExpression {
                    type_id: TypeId(type_id),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::String(decoded)),
                    span,
                })
            }
        }
    }

    fn resolve_integer_literal(
        &self,
        lexeme: &str,
        span: SourceSpan,
        expected_type: Option<usize>,
        is_negative_context: bool,
    ) -> Result<(usize, String), CompileFailure> {
        let canonical_str = canonical_integer_string(lexeme);

        let target_type = if let Some(exp_tid) = expected_type {
            if is_integer_or_dynamic_type(&self.analyzer.types[exp_tid]) {
                exp_tid
            } else {
                self.analyzer.name_to_type["int"]
            }
        } else {
            self.analyzer.name_to_type["int"]
        };

        let target_native = match &self.analyzer.types[target_type] {
            SemanticType::Native(nt) => nt,
            _ => &NativeType::Int,
        };

        if !is_integer_representable_in_native(&canonical_str, target_native, is_negative_context) {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::NumericLiteralNotRepresentable {
                        expected: clone_type_descriptor(
                            &self.analyzer.type_descriptors[target_type],
                        ),
                    },
                )),
                source_span: span,
            });
        }

        Ok((target_type, canonical_str))
    }

    fn analyze_identifier(
        &self,
        id: &Identifier<'source>,
    ) -> Result<SemanticExpression, CompileFailure> {
        if let Some(&bid) = self.name_to_binding.get(id.lexeme) {
            let type_id = self.bindings[bid].type_id.0;
            Ok(SemanticExpression {
                type_id: TypeId(type_id),
                kind: SemanticExpressionKind::Binding(BindingId(bid)),
                span: id.span,
            })
        } else if self.name_to_sig_binding.contains_key(id.lexeme) {
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Resolution(
                    ResolutionFailure::UnknownValueSymbol {
                        name: id.lexeme.into(),
                    },
                )),
                source_span: id.span,
            })
        } else {
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Resolution(
                    ResolutionFailure::UnknownValueSymbol {
                        name: id.lexeme.into(),
                    },
                )),
                source_span: id.span,
            })
        }
    }

    fn analyze_unary(
        &mut self,
        operator: &UnaryOperator,
        operand: &Expression<'source>,
        span: SourceSpan,
        expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        match operator {
            UnaryOperator::Not => {
                let op_expr = self.analyze_expression(operand, None)?;
                let bool_id = self.analyzer.name_to_type["bool"];
                if op_expr.type_id.0 != bool_id {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::UnaryOperator {
                                operator: clone_unary_operator(operator),
                                operand: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[op_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }
                Ok(SemanticExpression {
                    type_id: TypeId(bool_id),
                    kind: SemanticExpressionKind::Unary {
                        operator: clone_unary_operator(operator),
                        operand: Box::new(op_expr),
                    },
                    span,
                })
            }
            UnaryOperator::Negate => {
                // If operand is integer literal
                if let ExpressionKind::Literal {
                    kind: LiteralKind::Integer,
                    lexeme,
                } = &operand.kind
                {
                    let (type_id, canonical_str) =
                        self.resolve_integer_literal(lexeme, operand.span, expected_type, true)?;

                    let is_signed_or_dyn = match &self.analyzer.types[type_id] {
                        SemanticType::Native(nt) => is_signed_integer_or_dyn_native(nt),
                        _ => false,
                    };

                    if !is_signed_or_dyn {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                                TypeCheckingFailure::UnaryOperator {
                                    operator: clone_unary_operator(operator),
                                    operand: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[type_id],
                                    ),
                                },
                            )),
                            source_span: span,
                        });
                    }

                    let op_expr = SemanticExpression {
                        type_id: TypeId(type_id),
                        kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                            canonical_str,
                        )),
                        span: operand.span,
                    };

                    let out_type = if let Some(exp_tid) = expected_type {
                        if exp_tid == self.analyzer.name_to_type["dynamic"] {
                            self.analyzer.name_to_type["dynamic"]
                        } else {
                            type_id
                        }
                    } else {
                        type_id
                    };

                    return Ok(SemanticExpression {
                        type_id: TypeId(out_type),
                        kind: SemanticExpressionKind::Unary {
                            operator: clone_unary_operator(operator),
                            operand: Box::new(op_expr),
                        },
                        span,
                    });
                }

                let op_expr = self.analyze_expression(operand, expected_type)?;
                let is_signed_or_float_or_dyn = match &self.analyzer.types[op_expr.type_id.0] {
                    SemanticType::Native(nt) => matches!(
                        nt,
                        NativeType::Int
                            | NativeType::Int8
                            | NativeType::Int16
                            | NativeType::Int32
                            | NativeType::Int64
                            | NativeType::Int128
                            | NativeType::Float
                            | NativeType::Float32
                            | NativeType::Float64
                            | NativeType::Dynamic
                    ),
                    _ => false,
                };

                if !is_signed_or_float_or_dyn {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::UnaryOperator {
                                operator: clone_unary_operator(operator),
                                operand: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[op_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }

                let out_type = if let Some(exp_tid) = expected_type {
                    if exp_tid == self.analyzer.name_to_type["dynamic"] {
                        self.analyzer.name_to_type["dynamic"]
                    } else {
                        op_expr.type_id.0
                    }
                } else {
                    op_expr.type_id.0
                };

                Ok(SemanticExpression {
                    type_id: TypeId(out_type),
                    kind: SemanticExpressionKind::Unary {
                        operator: clone_unary_operator(operator),
                        operand: Box::new(op_expr),
                    },
                    span,
                })
            }
        }
    }

    fn is_statically_known_floating_result(&self, sem_expr: &SemanticExpression) -> bool {
        let tid = sem_expr.type_id.0;
        match &self.analyzer.types[tid] {
            SemanticType::Native(NativeType::Float | NativeType::Float32 | NativeType::Float64) => {
                true
            }
            SemanticType::Native(NativeType::Dynamic) => match &sem_expr.kind {
                SemanticExpressionKind::Literal(lit) => {
                    matches!(lit, SemanticLiteral::Floating(_))
                }
                SemanticExpressionKind::Binding(bid) => {
                    let b_tid = self.bindings[bid.0].type_id.0;
                    matches!(
                        &self.analyzer.types[b_tid],
                        SemanticType::Native(
                            NativeType::Float | NativeType::Float32 | NativeType::Float64
                        )
                    )
                }
                SemanticExpressionKind::Unary { operator, operand } => match operator {
                    UnaryOperator::Negate => self.is_statically_known_floating_result(operand),
                    UnaryOperator::Not => false,
                },
                SemanticExpressionKind::Binary {
                    left,
                    operator,
                    right,
                } => match operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Remainder => {
                        self.is_statically_known_floating_result(left)
                            || self.is_statically_known_floating_result(right)
                    }
                    _ => false,
                },
                SemanticExpressionKind::When(when) => when
                    .branches
                    .iter()
                    .any(|b| self.is_statically_known_floating_result(&b.result)),
                _ => false,
            },
            _ => false,
        }
    }

    fn analyze_binary(
        &mut self,
        left: &Expression<'source>,
        operator: &BinaryOperator,
        right: &Expression<'source>,
        span: SourceSpan,
        expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        let is_left_lit = matches!(left.kind, ExpressionKind::Literal { .. });
        let is_right_lit = matches!(right.kind, ExpressionKind::Literal { .. });

        let (left_expr, right_expr) = if !is_left_lit && is_right_lit {
            let l = self.analyze_expression(left, expected_type)?;
            let r = self.analyze_expression(right, Some(l.type_id.0))?;
            (l, r)
        } else if is_left_lit && !is_right_lit {
            let r = self.analyze_expression(right, expected_type)?;
            let l = self.analyze_expression(left, Some(r.type_id.0))?;
            (l, r)
        } else if is_left_lit && is_right_lit {
            let l = self.analyze_expression(left, expected_type)?;
            let r = self.analyze_expression(right, Some(l.type_id.0))?;
            (l, r)
        } else {
            let l = self.analyze_expression(left, expected_type)?;
            let r_expected = if let Some(exp_tid) = expected_type {
                if exp_tid == self.analyzer.name_to_type["dynamic"] {
                    Some(exp_tid)
                } else {
                    Some(l.type_id.0)
                }
            } else {
                Some(l.type_id.0)
            };
            let r = self.analyze_expression(right, r_expected)?;
            (l, r)
        };

        match operator {
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide => {
                let left_type = &self.analyzer.types[left_expr.type_id.0];
                let right_type = &self.analyzer.types[right_expr.type_id.0];
                let dynamic_id = self.analyzer.name_to_type["dynamic"];

                let (is_valid, out_type) = if left_expr.type_id.0 == right_expr.type_id.0 {
                    let is_num = match left_type {
                        SemanticType::Native(nt) => is_numeric_native(nt),
                        _ => false,
                    };
                    if !is_num {
                        (false, 0)
                    } else if expected_type == Some(dynamic_id) {
                        (true, dynamic_id)
                    } else {
                        (true, left_expr.type_id.0)
                    }
                } else if left_expr.type_id.0 == dynamic_id {
                    let is_r_num = match right_type {
                        SemanticType::Native(nt) => is_numeric_native(nt),
                        _ => false,
                    };
                    (is_r_num, dynamic_id)
                } else if right_expr.type_id.0 == dynamic_id {
                    let is_l_num = match left_type {
                        SemanticType::Native(nt) => is_numeric_native(nt),
                        _ => false,
                    };
                    (is_l_num, dynamic_id)
                } else {
                    (false, 0)
                };

                if !is_valid {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::ArithmeticOperator {
                                operator: clone_binary_operator(operator),
                                left: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[left_expr.type_id.0],
                                ),
                                right: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[right_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }

                Ok(SemanticExpression {
                    type_id: TypeId(out_type),
                    kind: SemanticExpressionKind::Binary {
                        left: Box::new(left_expr),
                        operator: clone_binary_operator(operator),
                        right: Box::new(right_expr),
                    },
                    span,
                })
            }
            BinaryOperator::Remainder => {
                let left_type = &self.analyzer.types[left_expr.type_id.0];
                let right_type = &self.analyzer.types[right_expr.type_id.0];
                let dynamic_id = self.analyzer.name_to_type["dynamic"];

                let left_is_floating = self.is_statically_known_floating_result(&left_expr);
                let right_is_floating = self.is_statically_known_floating_result(&right_expr);

                let (is_valid, out_type) = if left_is_floating || right_is_floating {
                    (false, 0)
                } else if left_expr.type_id.0 == right_expr.type_id.0 {
                    if left_expr.type_id.0 == dynamic_id {
                        (true, dynamic_id)
                    } else {
                        let is_int = match left_type {
                            SemanticType::Native(nt) => is_integer_native(nt),
                            _ => false,
                        };
                        if !is_int {
                            (false, 0)
                        } else if expected_type == Some(dynamic_id) {
                            (true, dynamic_id)
                        } else {
                            (true, left_expr.type_id.0)
                        }
                    }
                } else if left_expr.type_id.0 == dynamic_id {
                    let is_r_num = match right_type {
                        SemanticType::Native(nt) => is_numeric_native(nt),
                        _ => false,
                    };
                    (is_r_num, dynamic_id)
                } else if right_expr.type_id.0 == dynamic_id {
                    let is_l_num = match left_type {
                        SemanticType::Native(nt) => is_numeric_native(nt),
                        _ => false,
                    };
                    (is_l_num, dynamic_id)
                } else {
                    (false, 0)
                };

                if !is_valid {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::ArithmeticOperator {
                                operator: clone_binary_operator(operator),
                                left: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[left_expr.type_id.0],
                                ),
                                right: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[right_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }

                Ok(SemanticExpression {
                    type_id: TypeId(out_type),
                    kind: SemanticExpressionKind::Binary {
                        left: Box::new(left_expr),
                        operator: clone_binary_operator(operator),
                        right: Box::new(right_expr),
                    },
                    span,
                })
            }
            BinaryOperator::And | BinaryOperator::Or => {
                let bool_id = self.analyzer.name_to_type["bool"];
                if left_expr.type_id.0 != bool_id || right_expr.type_id.0 != bool_id {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::LogicalOperator {
                                operator: clone_binary_operator(operator),
                                left: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[left_expr.type_id.0],
                                ),
                                right: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[right_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }

                Ok(SemanticExpression {
                    type_id: TypeId(bool_id),
                    kind: SemanticExpressionKind::Binary {
                        left: Box::new(left_expr),
                        operator: clone_binary_operator(operator),
                        right: Box::new(right_expr),
                    },
                    span,
                })
            }
            BinaryOperator::Equal | BinaryOperator::NotEqual => {
                let is_comparable_for_equality = self
                    .analyzer
                    .is_type_equality_comparable(left_expr.type_id.0);

                if !is_comparable_for_equality || left_expr.type_id.0 != right_expr.type_id.0 {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::Comparison {
                                operator: clone_binary_operator(operator),
                                left: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[left_expr.type_id.0],
                                ),
                                right: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[right_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }

                let bool_id = self.analyzer.name_to_type["bool"];
                Ok(SemanticExpression {
                    type_id: TypeId(bool_id),
                    kind: SemanticExpressionKind::Binary {
                        left: Box::new(left_expr),
                        operator: clone_binary_operator(operator),
                        right: Box::new(right_expr),
                    },
                    span,
                })
            }
            BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual => {
                let is_orderable = match &self.analyzer.types[left_expr.type_id.0] {
                    SemanticType::Native(nt) => {
                        is_numeric_native(nt) && !matches!(nt, NativeType::Dynamic)
                    }
                    _ => false, // struct, enum, string are not orderable
                };

                if !is_orderable || left_expr.type_id.0 != right_expr.type_id.0 {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::Comparison {
                                operator: clone_binary_operator(operator),
                                left: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[left_expr.type_id.0],
                                ),
                                right: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[right_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: span,
                    });
                }

                let bool_id = self.analyzer.name_to_type["bool"];
                Ok(SemanticExpression {
                    type_id: TypeId(bool_id),
                    kind: SemanticExpressionKind::Binary {
                        left: Box::new(left_expr),
                        operator: clone_binary_operator(operator),
                        right: Box::new(right_expr),
                    },
                    span,
                })
            }
        }
    }

    fn analyze_field_access(
        &mut self,
        receiver: &Expression<'source>,
        field: &Identifier<'source>,
        span: SourceSpan,
    ) -> Result<SemanticExpression, CompileFailure> {
        let rec_expr = self.analyze_expression(receiver, None)?;

        let struct_meta = match self.analyzer.struct_metadata.get(&rec_expr.type_id.0) {
            Some(meta) => meta,
            None => {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::FieldAccessType {
                            actual: clone_type_descriptor(
                                &self.analyzer.type_descriptors[rec_expr.type_id.0],
                            ),
                        },
                    )),
                    source_span: rec_expr.span,
                });
            }
        };

        let field_idx = match struct_meta.name_to_field_idx.get(field.lexeme) {
            Some(&idx) => idx,
            None => {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::FieldNotFound {
                            field: field.lexeme.into(),
                        },
                    )),
                    source_span: field.span,
                });
            }
        };

        let field_type_id = struct_meta.fields[field_idx].type_id;

        Ok(SemanticExpression {
            type_id: TypeId(field_type_id),
            kind: SemanticExpressionKind::FieldAccess {
                receiver: Box::new(rec_expr),
                field: FieldId(field_idx),
            },
            span,
        })
    }

    fn analyze_struct_construction(
        &mut self,
        type_name: &Identifier<'source>,
        fields: &[FieldInitializer<'source>],
        span: SourceSpan,
    ) -> Result<SemanticExpression, CompileFailure> {
        let type_id = self.analyzer.resolve_type_identifier(type_name)?;

        let struct_meta = match self.analyzer.struct_metadata.get(&type_id) {
            Some(meta) => meta,
            None => {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::ExpectedStruct {
                            actual: clone_type_descriptor(&self.analyzer.type_descriptors[type_id]),
                        },
                    )),
                    source_span: type_name.span,
                });
            }
        };

        // Check duplicate initializers
        let mut seen_fields = HashSet::new();
        for f in fields {
            if seen_fields.contains(f.name.lexeme) {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::DuplicateFieldInitializer {
                            field: f.name.lexeme.into(),
                        },
                    )),
                    source_span: f.name.span,
                });
            }
            seen_fields.insert(f.name.lexeme);
        }

        // Check field existence
        for f in fields {
            if !struct_meta.name_to_field_idx.contains_key(f.name.lexeme) {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::FieldNotFound {
                            field: f.name.lexeme.into(),
                        },
                    )),
                    source_span: f.name.span,
                });
            }
        }

        // Check missing required fields
        for f_meta in &struct_meta.fields {
            if !seen_fields.contains(f_meta.name.as_str()) {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::MissingField {
                            field: f_meta.name.clone().into_boxed_str(),
                        },
                    )),
                    source_span: span,
                });
            }
        }

        // Analyze values and construct SemanticFieldValue list in source evaluation order
        let mut sem_fields = Vec::new();
        for f in fields {
            let f_idx = struct_meta.name_to_field_idx[f.name.lexeme];
            let expected_f_type = struct_meta.fields[f_idx].type_id;
            let val_expr = self.analyze_expression(&f.value, Some(expected_f_type))?;

            if !self
                .analyzer
                .is_value_type_compatible(val_expr.type_id.0, expected_f_type)
            {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::FieldTypeMismatch {
                            field: f.name.lexeme.into(),
                            expected: clone_type_descriptor(
                                &self.analyzer.type_descriptors[expected_f_type],
                            ),
                            actual: clone_type_descriptor(
                                &self.analyzer.type_descriptors[val_expr.type_id.0],
                            ),
                        },
                    )),
                    source_span: val_expr.span,
                });
            }

            sem_fields.push(SemanticFieldValue {
                field: FieldId(f_idx),
                value: val_expr,
            });
        }

        Ok(SemanticExpression {
            type_id: TypeId(type_id),
            kind: SemanticExpressionKind::StructConstruction { fields: sem_fields },
            span,
        })
    }

    fn analyze_enum_construction(
        &mut self,
        ec: &EnumConstruction<'source>,
        span: SourceSpan,
    ) -> Result<SemanticExpression, CompileFailure> {
        let (variant_qname, actual_shape) = match ec {
            EnumConstruction::Simple { variant } => (variant, EnumPayloadShape::Simple),
            EnumConstruction::Associated { variant, .. } => (variant, EnumPayloadShape::Associated),
            EnumConstruction::Structured { variant, .. } => (variant, EnumPayloadShape::Structured),
        };

        let type_id = self
            .analyzer
            .resolve_type_identifier(&variant_qname.qualifier)?;

        let enum_meta = match self.analyzer.enum_metadata.get(&type_id) {
            Some(meta) => meta,
            None => {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::ExpectedEnum {
                            actual: clone_type_descriptor(&self.analyzer.type_descriptors[type_id]),
                        },
                    )),
                    source_span: variant_qname.qualifier.span,
                });
            }
        };

        let var_idx = match enum_meta.name_to_variant_idx.get(variant_qname.name.lexeme) {
            Some(&idx) => idx,
            None => {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                        CompositeFailure::VariantNotFound {
                            variant: variant_qname.name.lexeme.into(),
                        },
                    )),
                    source_span: variant_qname.name.span,
                });
            }
        };

        let var_meta = &enum_meta.variants[var_idx];
        let expected_shape = var_meta.payload_shape();

        if !payload_shapes_match(&expected_shape, &actual_shape) {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                    CompositeFailure::VariantPayloadShapeMismatch {
                        expected: expected_shape,
                        actual: actual_shape,
                    },
                )),
                source_span: variant_qname.name.span,
            });
        }

        let payload = match (var_meta, ec) {
            (VariantMeta::Simple { .. }, EnumConstruction::Simple { .. }) => {
                SemanticEnumPayload::Simple
            }
            (
                VariantMeta::Associated {
                    type_id: exp_type, ..
                },
                EnumConstruction::Associated { value, .. },
            ) => {
                let val_expr = self.analyze_expression(value, Some(*exp_type))?;
                if !self
                    .analyzer
                    .is_value_type_compatible(val_expr.type_id.0, *exp_type)
                {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                            CompositeFailure::AssociatedPayloadTypeMismatch {
                                expected: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[*exp_type],
                                ),
                                actual: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[val_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: val_expr.span,
                    });
                }
                SemanticEnumPayload::Associated {
                    value: Box::new(val_expr),
                }
            }
            (
                VariantMeta::Structured {
                    fields: def_fields,
                    name_to_field_idx,
                    ..
                },
                EnumConstruction::Structured {
                    fields: init_fields,
                    ..
                },
            ) => {
                let mut seen = HashSet::new();
                for f in init_fields {
                    if seen.contains(f.name.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                                CompositeFailure::DuplicateFieldInitializer {
                                    field: f.name.lexeme.into(),
                                },
                            )),
                            source_span: f.name.span,
                        });
                    }
                    seen.insert(f.name.lexeme);
                }

                for f in init_fields {
                    if !name_to_field_idx.contains_key(f.name.lexeme) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                                CompositeFailure::FieldNotFound {
                                    field: f.name.lexeme.into(),
                                },
                            )),
                            source_span: f.name.span,
                        });
                    }
                }

                for def_f in def_fields {
                    if !seen.contains(def_f.name.as_str()) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                                CompositeFailure::MissingField {
                                    field: def_f.name.clone().into_boxed_str(),
                                },
                            )),
                            source_span: span,
                        });
                    }
                }

                let mut sem_fields = Vec::new();
                for f in init_fields {
                    let f_idx = name_to_field_idx[f.name.lexeme];
                    let exp_f_type = def_fields[f_idx].type_id;
                    let val_expr = self.analyze_expression(&f.value, Some(exp_f_type))?;

                    if !self
                        .analyzer
                        .is_value_type_compatible(val_expr.type_id.0, exp_f_type)
                    {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                                CompositeFailure::FieldTypeMismatch {
                                    field: f.name.lexeme.into(),
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[exp_f_type],
                                    ),
                                    actual: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[val_expr.type_id.0],
                                    ),
                                },
                            )),
                            source_span: val_expr.span,
                        });
                    }

                    sem_fields.push(SemanticFieldValue {
                        field: FieldId(f_idx),
                        value: val_expr,
                    });
                }

                SemanticEnumPayload::Structured { fields: sem_fields }
            }
            _ => unreachable!(),
        };

        Ok(SemanticExpression {
            type_id: TypeId(type_id),
            kind: SemanticExpressionKind::EnumConstruction {
                variant: VariantId(var_idx),
                payload,
            },
            span,
        })
    }

    fn analyze_function_call(
        &mut self,
        call: &FunctionCall<'source>,
        span: SourceSpan,
        _expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        let callee_name = call.callee.lexeme;

        // Check if official conversion
        if let Some(target_native) = get_conversion_target_native(callee_name) {
            if call.arguments.len() != 1 {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                        CallFailure::ArityMismatch {
                            expected: 1,
                            actual: call.arguments.len(),
                        },
                    )),
                    source_span: span,
                });
            }

            let operand = self.analyze_expression(&call.arguments[0], None)?;
            let target_type_id = self.analyzer.name_to_type[native_type_name(&target_native)];

            if !is_valid_conversion(&self.analyzer.types[operand.type_id.0], &target_native) {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                        TypeCheckingFailure::InvalidConversion {
                            source: clone_type_descriptor(
                                &self.analyzer.type_descriptors[operand.type_id.0],
                            ),
                            target: clone_type_descriptor(
                                &self.analyzer.type_descriptors[target_type_id],
                            ),
                        },
                    )),
                    source_span: span,
                });
            }

            return Ok(SemanticExpression {
                type_id: TypeId(target_type_id),
                kind: SemanticExpressionKind::Conversion {
                    operand: Box::new(operand),
                },
                span,
            });
        }

        // Call target resolution:
        // Candidates:
        // 1. Local function
        // 2. Signature Dependency parameter
        // 3. Direct imported signatures
        let is_local_fn = self.analyzer.name_to_function.get(callee_name).copied();
        let is_sig_dep = self.name_to_sig_binding.get(callee_name).copied();
        let direct_sigs = self
            .analyzer
            .name_to_signatures
            .get(callee_name)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);

        let total_candidates = (if is_local_fn.is_some() { 1 } else { 0 })
            + (if is_sig_dep.is_some() { 1 } else { 0 })
            + direct_sigs.len();

        if total_candidates > 1 {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                    CallFailure::AmbiguousTarget {
                        name: callee_name.into(),
                    },
                )),
                source_span: call.callee.span,
            });
        }

        if total_candidates == 0 {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                    CallFailure::FunctionNotFound {
                        name: callee_name.into(),
                    },
                )),
                source_span: call.callee.span,
            });
        }

        let (target, formal_params, result_type) = if let Some(fid) = is_local_fn {
            let header = &self.analyzer.function_headers[fid];
            let formal_params: Vec<_> = header
                .parameters
                .iter()
                .map(|p| match p {
                    FormalParamMeta::Value { type_id, .. } => {
                        SemanticSignatureParameter::Value(TypeId(*type_id))
                    }
                    FormalParamMeta::SignatureDependency { signature_id, .. } => {
                        SemanticSignatureParameter::SignatureDependency(SignatureId(*signature_id))
                    }
                })
                .collect();
            (
                SemanticCallTarget::Internal(FunctionId(fid)),
                formal_params,
                header.result_type,
            )
        } else if let Some(sbid) = is_sig_dep {
            let sid = self.signature_bindings[sbid].signature.0;
            let sig = &self.analyzer.signatures[sid];
            let formal_params: Vec<_> = sig
                .parameters
                .iter()
                .map(|p| match p {
                    SemanticSignatureParameter::Value(tid) => {
                        SemanticSignatureParameter::Value(TypeId(tid.0))
                    }
                    SemanticSignatureParameter::SignatureDependency(sid) => {
                        SemanticSignatureParameter::SignatureDependency(SignatureId(sid.0))
                    }
                })
                .collect();
            (
                SemanticCallTarget::SignatureDependency(SignatureBindingId(sbid)),
                formal_params,
                sig.result_type.0,
            )
        } else if direct_sigs.len() == 1 {
            let sid = direct_sigs[0];
            let sig = &self.analyzer.signatures[sid];
            let formal_params: Vec<_> = sig
                .parameters
                .iter()
                .map(|p| match p {
                    SemanticSignatureParameter::Value(tid) => {
                        SemanticSignatureParameter::Value(TypeId(tid.0))
                    }
                    SemanticSignatureParameter::SignatureDependency(sid) => {
                        SemanticSignatureParameter::SignatureDependency(SignatureId(sid.0))
                    }
                })
                .collect();
            (
                SemanticCallTarget::DirectSignature(SignatureId(sid)),
                formal_params,
                sig.result_type.0,
            )
        } else {
            unreachable!()
        };

        if call.arguments.len() != formal_params.len() {
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                    CallFailure::ArityMismatch {
                        expected: formal_params.len(),
                        actual: call.arguments.len(),
                    },
                )),
                source_span: span,
            });
        }

        let mut sem_args = Vec::new();
        for (i, (arg_expr, formal_p)) in call.arguments.iter().zip(&formal_params).enumerate() {
            match formal_p {
                SemanticSignatureParameter::Value(exp_type) => {
                    // If the argument is an identifier referencing a Signature Dependency, that is ArgumentKindMismatch
                    if let ExpressionKind::Identifier(id) = &arg_expr.kind {
                        if self.name_to_sig_binding.contains_key(id.lexeme) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                    CallFailure::ArgumentKindMismatch {
                                        position: i,
                                        expected: SemanticArgumentKind::Value,
                                        actual: SemanticArgumentKind::SignatureDependency,
                                    },
                                )),
                                source_span: arg_expr.span,
                            });
                        }
                    }

                    let val_expr = self.analyze_expression(arg_expr, Some(exp_type.0))?;
                    if !self
                        .analyzer
                        .is_value_type_compatible(val_expr.type_id.0, exp_type.0)
                    {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                CallFailure::ArgumentTypeMismatch {
                                    position: i,
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[exp_type.0],
                                    ),
                                    actual: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[val_expr.type_id.0],
                                    ),
                                },
                            )),
                            source_span: val_expr.span,
                        });
                    }
                    sem_args.push(SemanticArgument::Value(val_expr));
                }
                SemanticSignatureParameter::SignatureDependency(exp_sig_id) => {
                    // Must be an identifier referencing a local Signature Dependency
                    let sbid = if let ExpressionKind::Identifier(id) = &arg_expr.kind {
                        if let Some(&sbid) = self.name_to_sig_binding.get(id.lexeme) {
                            sbid
                        } else {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                    CallFailure::ArgumentKindMismatch {
                                        position: i,
                                        expected: SemanticArgumentKind::SignatureDependency,
                                        actual: SemanticArgumentKind::Value,
                                    },
                                )),
                                source_span: arg_expr.span,
                            });
                        }
                    } else {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                CallFailure::ArgumentKindMismatch {
                                    position: i,
                                    expected: SemanticArgumentKind::SignatureDependency,
                                    actual: SemanticArgumentKind::Value,
                                },
                            )),
                            source_span: arg_expr.span,
                        });
                    };

                    let actual_sig_id = self.signature_bindings[sbid].signature.0;
                    if actual_sig_id != exp_sig_id.0 {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                CallFailure::SignatureDependencyMismatch {
                                    position: i,
                                    expected: clone_signature_symbol(
                                        &self.analyzer.signatures[exp_sig_id.0].symbol,
                                    ),
                                    actual: clone_signature_symbol(
                                        &self.analyzer.signatures[actual_sig_id].symbol,
                                    ),
                                },
                            )),
                            source_span: arg_expr.span,
                        });
                    }

                    sem_args.push(SemanticArgument::SignatureDependency(SignatureBindingId(
                        sbid,
                    )));
                }
            }
        }

        Ok(SemanticExpression {
            type_id: TypeId(result_type),
            kind: SemanticExpressionKind::Call(SemanticCall {
                target,
                arguments: sem_args,
            }),
            span,
        })
    }

    fn analyze_pipeline(
        &mut self,
        pipe: &Pipeline<'source>,
        expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        let mut current_expr = self.analyze_expression(&pipe.source, None)?;

        for (stage_idx, stage) in pipe.stages.iter().enumerate() {
            let is_last_stage = stage_idx == pipe.stages.len() - 1;
            let stage_expected_type = if is_last_stage { expected_type } else { None };

            let callee_name = stage.callee.lexeme;
            let stage_span = SourceSpan {
                start: current_expr.span.start,
                end: stage.callee.span.end,
            };

            // Check if conversion
            if let Some(target_native) = get_conversion_target_native(callee_name) {
                if !stage.additional_arguments.is_empty() {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                            CallFailure::ArityMismatch {
                                expected: 1,
                                actual: 1 + stage.additional_arguments.len(),
                            },
                        )),
                        source_span: stage_span,
                    });
                }

                let target_type_id = self.analyzer.name_to_type[native_type_name(&target_native)];

                if !is_valid_conversion(
                    &self.analyzer.types[current_expr.type_id.0],
                    &target_native,
                ) {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                            TypeCheckingFailure::InvalidConversion {
                                source: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[current_expr.type_id.0],
                                ),
                                target: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[target_type_id],
                                ),
                            },
                        )),
                        source_span: stage_span,
                    });
                }

                current_expr = SemanticExpression {
                    type_id: TypeId(target_type_id),
                    kind: SemanticExpressionKind::Conversion {
                        operand: Box::new(current_expr),
                    },
                    span: stage_span,
                };
                continue;
            }

            // Normal function/signature stage
            let is_local_fn = self.analyzer.name_to_function.get(callee_name).copied();
            let is_sig_dep = self.name_to_sig_binding.get(callee_name).copied();
            let direct_sigs = self
                .analyzer
                .name_to_signatures
                .get(callee_name)
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            let total_candidates = (if is_local_fn.is_some() { 1 } else { 0 })
                + (if is_sig_dep.is_some() { 1 } else { 0 })
                + direct_sigs.len();

            if total_candidates > 1 {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                        CallFailure::AmbiguousTarget {
                            name: callee_name.into(),
                        },
                    )),
                    source_span: stage.callee.span,
                });
            }

            if total_candidates == 0 {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                        CallFailure::FunctionNotFound {
                            name: callee_name.into(),
                        },
                    )),
                    source_span: stage.callee.span,
                });
            }

            let (target, formal_params, result_type) = if let Some(fid) = is_local_fn {
                let header = &self.analyzer.function_headers[fid];
                let formal_params: Vec<_> = header
                    .parameters
                    .iter()
                    .map(|p| match p {
                        FormalParamMeta::Value { type_id, .. } => {
                            SemanticSignatureParameter::Value(TypeId(*type_id))
                        }
                        FormalParamMeta::SignatureDependency { signature_id, .. } => {
                            SemanticSignatureParameter::SignatureDependency(SignatureId(
                                *signature_id,
                            ))
                        }
                    })
                    .collect();
                (
                    SemanticCallTarget::Internal(FunctionId(fid)),
                    formal_params,
                    header.result_type,
                )
            } else if let Some(sbid) = is_sig_dep {
                let sid = self.signature_bindings[sbid].signature.0;
                let sig = &self.analyzer.signatures[sid];
                let formal_params: Vec<_> = sig
                    .parameters
                    .iter()
                    .map(|p| match p {
                        SemanticSignatureParameter::Value(tid) => {
                            SemanticSignatureParameter::Value(TypeId(tid.0))
                        }
                        SemanticSignatureParameter::SignatureDependency(sid) => {
                            SemanticSignatureParameter::SignatureDependency(SignatureId(sid.0))
                        }
                    })
                    .collect();
                (
                    SemanticCallTarget::SignatureDependency(SignatureBindingId(sbid)),
                    formal_params,
                    sig.result_type.0,
                )
            } else if direct_sigs.len() == 1 {
                let sid = direct_sigs[0];
                let sig = &self.analyzer.signatures[sid];
                let formal_params: Vec<_> = sig
                    .parameters
                    .iter()
                    .map(|p| match p {
                        SemanticSignatureParameter::Value(tid) => {
                            SemanticSignatureParameter::Value(TypeId(tid.0))
                        }
                        SemanticSignatureParameter::SignatureDependency(sid) => {
                            SemanticSignatureParameter::SignatureDependency(SignatureId(sid.0))
                        }
                    })
                    .collect();
                (
                    SemanticCallTarget::DirectSignature(SignatureId(sid)),
                    formal_params,
                    sig.result_type.0,
                )
            } else {
                unreachable!()
            };

            let total_actual_args = 1 + stage.additional_arguments.len();
            if total_actual_args != formal_params.len() {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                        CallFailure::ArityMismatch {
                            expected: formal_params.len(),
                            actual: total_actual_args,
                        },
                    )),
                    source_span: stage_span,
                });
            }

            let mut sem_args = Vec::new();

            // First argument is current_expr
            match &formal_params[0] {
                SemanticSignatureParameter::Value(exp_type) => {
                    if !self
                        .analyzer
                        .is_value_type_compatible(current_expr.type_id.0, exp_type.0)
                    {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                CallFailure::ArgumentTypeMismatch {
                                    position: 0,
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[exp_type.0],
                                    ),
                                    actual: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[current_expr.type_id.0],
                                    ),
                                },
                            )),
                            source_span: current_expr.span,
                        });
                    }
                    sem_args.push(SemanticArgument::Value(current_expr));
                }
                SemanticSignatureParameter::SignatureDependency(_) => {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                            CallFailure::ArgumentKindMismatch {
                                position: 0,
                                expected: SemanticArgumentKind::SignatureDependency,
                                actual: SemanticArgumentKind::Value,
                            },
                        )),
                        source_span: current_expr.span,
                    });
                }
            }

            // Additional arguments
            for (i, (arg_expr, formal_p)) in stage
                .additional_arguments
                .iter()
                .zip(&formal_params[1..])
                .enumerate()
            {
                let pos = i + 1;
                match formal_p {
                    SemanticSignatureParameter::Value(exp_type) => {
                        if let ExpressionKind::Identifier(id) = &arg_expr.kind {
                            if self.name_to_sig_binding.contains_key(id.lexeme) {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                        CallFailure::ArgumentKindMismatch {
                                            position: pos,
                                            expected: SemanticArgumentKind::Value,
                                            actual: SemanticArgumentKind::SignatureDependency,
                                        },
                                    )),
                                    source_span: arg_expr.span,
                                });
                            }
                        }

                        let val_expr = self.analyze_expression(arg_expr, Some(exp_type.0))?;
                        if !self
                            .analyzer
                            .is_value_type_compatible(val_expr.type_id.0, exp_type.0)
                        {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                    CallFailure::ArgumentTypeMismatch {
                                        position: pos,
                                        expected: clone_type_descriptor(
                                            &self.analyzer.type_descriptors[exp_type.0],
                                        ),
                                        actual: clone_type_descriptor(
                                            &self.analyzer.type_descriptors[val_expr.type_id.0],
                                        ),
                                    },
                                )),
                                source_span: val_expr.span,
                            });
                        }
                        sem_args.push(SemanticArgument::Value(val_expr));
                    }
                    SemanticSignatureParameter::SignatureDependency(exp_sig_id) => {
                        let sbid = if let ExpressionKind::Identifier(id) = &arg_expr.kind {
                            if let Some(&sbid) = self.name_to_sig_binding.get(id.lexeme) {
                                sbid
                            } else {
                                return Err(CompileFailure {
                                    kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                        CallFailure::ArgumentKindMismatch {
                                            position: pos,
                                            expected: SemanticArgumentKind::SignatureDependency,
                                            actual: SemanticArgumentKind::Value,
                                        },
                                    )),
                                    source_span: arg_expr.span,
                                });
                            }
                        } else {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                    CallFailure::ArgumentKindMismatch {
                                        position: pos,
                                        expected: SemanticArgumentKind::SignatureDependency,
                                        actual: SemanticArgumentKind::Value,
                                    },
                                )),
                                source_span: arg_expr.span,
                            });
                        };

                        let actual_sig_id = self.signature_bindings[sbid].signature.0;
                        if actual_sig_id != exp_sig_id.0 {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                                    CallFailure::SignatureDependencyMismatch {
                                        position: pos,
                                        expected: clone_signature_symbol(
                                            &self.analyzer.signatures[exp_sig_id.0].symbol,
                                        ),
                                        actual: clone_signature_symbol(
                                            &self.analyzer.signatures[actual_sig_id].symbol,
                                        ),
                                    },
                                )),
                                source_span: arg_expr.span,
                            });
                        }

                        sem_args.push(SemanticArgument::SignatureDependency(SignatureBindingId(
                            sbid,
                        )));
                    }
                }
            }

            let final_type = if is_last_stage {
                if let Some(_exp) = stage_expected_type {
                    // let enclosing context check
                }
                result_type
            } else {
                result_type
            };

            current_expr = SemanticExpression {
                type_id: TypeId(final_type),
                kind: SemanticExpressionKind::Call(SemanticCall {
                    target,
                    arguments: sem_args,
                }),
                span: stage_span,
            };
        }

        Ok(current_expr)
    }

    fn analyze_when(
        &mut self,
        when: &WhenExpression<'source>,
        span: SourceSpan,
        expected_type: Option<usize>,
    ) -> Result<SemanticExpression, CompileFailure> {
        let subject_expr = self.analyze_expression(&when.subject, None)?;

        let enum_meta = match self.analyzer.enum_metadata.get(&subject_expr.type_id.0) {
            Some(meta) => meta,
            None => {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::When(
                        WhenFailure::SubjectNotEnum {
                            actual: clone_type_descriptor(
                                &self.analyzer.type_descriptors[subject_expr.type_id.0],
                            ),
                        },
                    )),
                    source_span: subject_expr.span,
                });
            }
        };

        let mut visited_variants = HashSet::new();
        let mut branches = Vec::new();
        let mut common_result_type: Option<usize> = expected_type;

        for corr in &when.correspondences {
            let pat_variant = match &corr.pattern {
                WhenPattern::Simple { variant } => variant,
                WhenPattern::Associated { variant, .. } => variant,
                WhenPattern::Structured { variant, .. } => variant,
            };
            let pat_enum_id = self
                .analyzer
                .resolve_type_identifier(&pat_variant.qualifier)?;

            if pat_enum_id != subject_expr.type_id.0 {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::When(
                        WhenFailure::PatternEnumMismatch {
                            expected: clone_type_descriptor(
                                &self.analyzer.type_descriptors[subject_expr.type_id.0],
                            ),
                            actual: clone_type_descriptor(
                                &self.analyzer.type_descriptors[pat_enum_id],
                            ),
                        },
                    )),
                    source_span: pat_variant.qualifier.span,
                });
            }

            let var_idx = match enum_meta.name_to_variant_idx.get(pat_variant.name.lexeme) {
                Some(&idx) => idx,
                None => {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::When(
                            WhenFailure::VariantNotFound {
                                variant: pat_variant.name.lexeme.into(),
                            },
                        )),
                        source_span: pat_variant.name.span,
                    });
                }
            };

            if visited_variants.contains(pat_variant.name.lexeme) {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::When(
                        WhenFailure::DuplicateVariantCorrespondence {
                            variant: pat_variant.name.lexeme.into(),
                        },
                    )),
                    source_span: pat_variant.name.span,
                });
            }
            visited_variants.insert(pat_variant.name.lexeme);

            let var_meta = &enum_meta.variants[var_idx];
            let exp_shape = var_meta.payload_shape();
            let actual_shape = match &corr.pattern {
                WhenPattern::Simple { .. } => EnumPayloadShape::Simple,
                WhenPattern::Associated { .. } => EnumPayloadShape::Associated,
                WhenPattern::Structured { .. } => EnumPayloadShape::Structured,
            };

            if !payload_shapes_match(&exp_shape, &actual_shape) {
                return Err(CompileFailure {
                    kind: CompileFailureKind::Semantic(SemanticFailure::When(
                        WhenFailure::PayloadShapeMismatch {
                            expected: exp_shape,
                            actual: actual_shape,
                        },
                    )),
                    source_span: pat_variant.name.span,
                });
            }

            // Analyze extraction and setup branch-local scope
            let mut branch_bindings = Vec::new();

            let extraction = match (&corr.pattern, var_meta) {
                (WhenPattern::Simple { .. }, VariantMeta::Simple { .. }) => {
                    SemanticVariantExtraction::Simple
                }
                (
                    WhenPattern::Associated { binding, .. },
                    VariantMeta::Associated {
                        type_id: exp_payload_type,
                        ..
                    },
                ) => {
                    let bind_name = binding.name.lexeme;

                    if !is_valid_snake_case(bind_name) {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::InvalidNamingConvention {
                                    role: SemanticNameRole::Binding,
                                },
                            )),
                            source_span: binding.name.span,
                        });
                    }

                    if self.name_to_binding.contains_key(bind_name)
                        || self.name_to_sig_binding.contains_key(bind_name)
                    {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                DeclarationFailure::BindingNameCollision {
                                    name: bind_name.into(),
                                },
                            )),
                            source_span: binding.name.span,
                        });
                    }

                    let declared_tid = self.analyzer.resolve_type_identifier(&binding.type_name)?;
                    if declared_tid != *exp_payload_type {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Semantic(SemanticFailure::When(
                                WhenFailure::ExtractionTypeMismatch {
                                    expected: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[*exp_payload_type],
                                    ),
                                    actual: clone_type_descriptor(
                                        &self.analyzer.type_descriptors[declared_tid],
                                    ),
                                },
                            )),
                            source_span: binding.type_name.span,
                        });
                    }

                    let bid = self.bindings.len();
                    self.bindings.push(SemanticBinding {
                        type_id: TypeId(declared_tid),
                    });
                    self.name_to_binding.insert(bind_name.to_string(), bid);
                    branch_bindings.push(bind_name.to_string());

                    SemanticVariantExtraction::Associated {
                        binding: BindingId(bid),
                    }
                }
                (
                    WhenPattern::Structured {
                        fields: pat_fields, ..
                    },
                    VariantMeta::Structured {
                        fields: def_fields,
                        name_to_field_idx,
                        ..
                    },
                ) => {
                    let mut seen_fields = HashSet::new();
                    for pf in pat_fields {
                        if seen_fields.contains(pf.field.lexeme) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::When(
                                    WhenFailure::DuplicateField {
                                        field: pf.field.lexeme.into(),
                                    },
                                )),
                                source_span: pf.field.span,
                            });
                        }
                        seen_fields.insert(pf.field.lexeme);
                    }

                    for pf in pat_fields {
                        if !name_to_field_idx.contains_key(pf.field.lexeme) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::When(
                                    WhenFailure::FieldNotFound {
                                        field: pf.field.lexeme.into(),
                                    },
                                )),
                                source_span: pf.field.span,
                            });
                        }
                    }

                    for def_f in def_fields {
                        if !seen_fields.contains(def_f.name.as_str()) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::When(
                                    WhenFailure::MissingField {
                                        field: def_f.name.clone().into_boxed_str(),
                                    },
                                )),
                                source_span: pat_variant.name.span,
                            });
                        }
                    }

                    let mut sem_field_bindings = Vec::new();
                    for pf in pat_fields {
                        let bind_name = pf.binding.name.lexeme;

                        if !is_valid_snake_case(bind_name) {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                    DeclarationFailure::InvalidNamingConvention {
                                        role: SemanticNameRole::Binding,
                                    },
                                )),
                                source_span: pf.binding.name.span,
                            });
                        }

                        if self.name_to_binding.contains_key(bind_name)
                            || self.name_to_sig_binding.contains_key(bind_name)
                        {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::Declaration(
                                    DeclarationFailure::BindingNameCollision {
                                        name: bind_name.into(),
                                    },
                                )),
                                source_span: pf.binding.name.span,
                            });
                        }

                        let f_idx = name_to_field_idx[pf.field.lexeme];
                        let exp_f_type = def_fields[f_idx].type_id;
                        let decl_f_type = self
                            .analyzer
                            .resolve_type_identifier(&pf.binding.type_name)?;

                        if decl_f_type != exp_f_type {
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Semantic(SemanticFailure::When(
                                    WhenFailure::ExtractionTypeMismatch {
                                        expected: clone_type_descriptor(
                                            &self.analyzer.type_descriptors[exp_f_type],
                                        ),
                                        actual: clone_type_descriptor(
                                            &self.analyzer.type_descriptors[decl_f_type],
                                        ),
                                    },
                                )),
                                source_span: pf.binding.type_name.span,
                            });
                        }

                        let bid = self.bindings.len();
                        self.bindings.push(SemanticBinding {
                            type_id: TypeId(decl_f_type),
                        });
                        self.name_to_binding.insert(bind_name.to_string(), bid);
                        branch_bindings.push(bind_name.to_string());

                        sem_field_bindings.push((
                            f_idx,
                            SemanticFieldBinding {
                                field: FieldId(f_idx),
                                binding: BindingId(bid),
                            },
                        ));
                    }

                    sem_field_bindings.sort_by_key(|(idx, _)| *idx);
                    let fields = sem_field_bindings.into_iter().map(|(_, val)| val).collect();
                    SemanticVariantExtraction::Structured { fields }
                }
                _ => unreachable!(),
            };

            let res_expr = self.analyze_expression(&corr.result, common_result_type);

            // Clean up branch-local bindings from scope table (bindings Vec keeps them owned)
            for b_name in branch_bindings {
                self.name_to_binding.remove(&b_name);
            }

            let res_expr = res_expr?;

            if let Some(comm_tid) = common_result_type {
                if !self
                    .analyzer
                    .is_value_type_compatible(res_expr.type_id.0, comm_tid)
                {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Semantic(SemanticFailure::When(
                            WhenFailure::BranchResultTypeMismatch {
                                expected: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[comm_tid],
                                ),
                                actual: clone_type_descriptor(
                                    &self.analyzer.type_descriptors[res_expr.type_id.0],
                                ),
                            },
                        )),
                        source_span: res_expr.span,
                    });
                }
            } else {
                common_result_type = Some(res_expr.type_id.0);
            }

            branches.push(SemanticWhenBranch {
                variant: VariantId(var_idx),
                extraction,
                result: res_expr,
            });
        }

        // Check exhaustiveness in canonical order
        if visited_variants.len() != enum_meta.variants.len() {
            let mut missing = Vec::new();
            for v in &enum_meta.variants {
                if !visited_variants.contains(v.name()) {
                    missing.push(v.name().to_string().into_boxed_str());
                }
            }
            return Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::When(
                    WhenFailure::NonExhaustive { missing },
                )),
                source_span: span,
            });
        }

        let final_type_id =
            common_result_type.expect("enum has at least 1 variant, so at least 1 branch");

        Ok(SemanticExpression {
            type_id: TypeId(final_type_id),
            kind: SemanticExpressionKind::When(SemanticWhen {
                subject: Box::new(subject_expr),
                branches,
            }),
            span,
        })
    }
}

// --- Helpers ---

fn clone_native_type(nt: &NativeType) -> NativeType {
    match nt {
        NativeType::Int => NativeType::Int,
        NativeType::Float => NativeType::Float,
        NativeType::Bool => NativeType::Bool,
        NativeType::String => NativeType::String,
        NativeType::Dynamic => NativeType::Dynamic,
        NativeType::Int8 => NativeType::Int8,
        NativeType::Int16 => NativeType::Int16,
        NativeType::Int32 => NativeType::Int32,
        NativeType::Int64 => NativeType::Int64,
        NativeType::Int128 => NativeType::Int128,
        NativeType::Uint8 => NativeType::Uint8,
        NativeType::Uint16 => NativeType::Uint16,
        NativeType::Uint32 => NativeType::Uint32,
        NativeType::Uint64 => NativeType::Uint64,
        NativeType::Uint128 => NativeType::Uint128,
        NativeType::Float32 => NativeType::Float32,
        NativeType::Float64 => NativeType::Float64,
    }
}

fn clone_type_descriptor(d: &SemanticTypeDescriptor) -> SemanticTypeDescriptor {
    match d {
        SemanticTypeDescriptor::Native(nt) => SemanticTypeDescriptor::Native(clone_native_type(nt)),
        SemanticTypeDescriptor::Local(s) => SemanticTypeDescriptor::Local(s.clone()),
        SemanticTypeDescriptor::Shared(sym) => SemanticTypeDescriptor::Shared(TypeSymbol {
            module: sym.module.clone(),
            name: sym.name.clone(),
        }),
    }
}

fn clone_signature_symbol(sym: &SignatureSymbol) -> SignatureSymbol {
    SignatureSymbol {
        module: sym.module.clone(),
        name: sym.name.clone(),
    }
}

fn clone_unary_operator(op: &UnaryOperator) -> UnaryOperator {
    match op {
        UnaryOperator::Not => UnaryOperator::Not,
        UnaryOperator::Negate => UnaryOperator::Negate,
    }
}

fn clone_binary_operator(op: &BinaryOperator) -> BinaryOperator {
    match op {
        BinaryOperator::Multiply => BinaryOperator::Multiply,
        BinaryOperator::Divide => BinaryOperator::Divide,
        BinaryOperator::Remainder => BinaryOperator::Remainder,
        BinaryOperator::Add => BinaryOperator::Add,
        BinaryOperator::Subtract => BinaryOperator::Subtract,
        BinaryOperator::Less => BinaryOperator::Less,
        BinaryOperator::LessEqual => BinaryOperator::LessEqual,
        BinaryOperator::Greater => BinaryOperator::Greater,
        BinaryOperator::GreaterEqual => BinaryOperator::GreaterEqual,
        BinaryOperator::Equal => BinaryOperator::Equal,
        BinaryOperator::NotEqual => BinaryOperator::NotEqual,
        BinaryOperator::And => BinaryOperator::And,
        BinaryOperator::Or => BinaryOperator::Or,
    }
}

fn payload_shapes_match(a: &EnumPayloadShape, b: &EnumPayloadShape) -> bool {
    matches!(
        (a, b),
        (EnumPayloadShape::Simple, EnumPayloadShape::Simple)
            | (EnumPayloadShape::Associated, EnumPayloadShape::Associated)
            | (EnumPayloadShape::Structured, EnumPayloadShape::Structured)
    )
}

fn is_valid_pascal_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_uppercase() {
        return false;
    }
    for c in chars {
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }
    true
}

fn is_valid_snake_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_lowercase() {
        return false;
    }
    if s.ends_with('_') || s.contains("__") {
        return false;
    }
    for segment in s.split('_') {
        if segment.is_empty() || !segment.chars().next().unwrap().is_ascii_lowercase() {
            return false;
        }
        for c in segment.chars() {
            if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
                return false;
            }
        }
    }
    true
}

fn canonical_integer_string(lexeme: &str) -> String {
    let trimmed = lexeme.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn decode_string_literal(lexeme: &str) -> String {
    if lexeme.len() < 2 {
        return String::new();
    }
    let inner = &lexeme[1..lexeme.len() - 1];
    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_c) = chars.next() {
                match next_c {
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    other => {
                        result.push('\\');
                        result.push(other);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn is_integer_or_dynamic_type(sem_type: &SemanticType) -> bool {
    match sem_type {
        SemanticType::Native(nt) => matches!(
            nt,
            NativeType::Int
                | NativeType::Int8
                | NativeType::Int16
                | NativeType::Int32
                | NativeType::Int64
                | NativeType::Int128
                | NativeType::Uint8
                | NativeType::Uint16
                | NativeType::Uint32
                | NativeType::Uint64
                | NativeType::Uint128
                | NativeType::Dynamic
        ),
        _ => false,
    }
}

fn is_numeric_native(nt: &NativeType) -> bool {
    matches!(
        nt,
        NativeType::Int
            | NativeType::Int8
            | NativeType::Int16
            | NativeType::Int32
            | NativeType::Int64
            | NativeType::Int128
            | NativeType::Uint8
            | NativeType::Uint16
            | NativeType::Uint32
            | NativeType::Uint64
            | NativeType::Uint128
            | NativeType::Float
            | NativeType::Float32
            | NativeType::Float64
            | NativeType::Dynamic
    )
}

fn is_integer_native(nt: &NativeType) -> bool {
    matches!(
        nt,
        NativeType::Int
            | NativeType::Int8
            | NativeType::Int16
            | NativeType::Int32
            | NativeType::Int64
            | NativeType::Int128
            | NativeType::Uint8
            | NativeType::Uint16
            | NativeType::Uint32
            | NativeType::Uint64
            | NativeType::Uint128
    )
}

fn is_signed_integer_or_dyn_native(nt: &NativeType) -> bool {
    matches!(
        nt,
        NativeType::Int
            | NativeType::Int8
            | NativeType::Int16
            | NativeType::Int32
            | NativeType::Int64
            | NativeType::Int128
            | NativeType::Dynamic
    )
}

fn is_integer_or_dyn_native(nt: &NativeType) -> bool {
    matches!(
        nt,
        NativeType::Int
            | NativeType::Int8
            | NativeType::Int16
            | NativeType::Int32
            | NativeType::Int64
            | NativeType::Int128
            | NativeType::Uint8
            | NativeType::Uint16
            | NativeType::Uint32
            | NativeType::Uint64
            | NativeType::Uint128
            | NativeType::Dynamic
    )
}

fn is_integer_representable_in_native(
    canonical_str: &str,
    native: &NativeType,
    is_negative_context: bool,
) -> bool {
    if matches!(native, NativeType::Dynamic) {
        return true;
    }

    match native {
        NativeType::Int8 => {
            if let Ok(val) = canonical_str.parse::<u8>() {
                if is_negative_context {
                    val <= 128
                } else {
                    val <= 127
                }
            } else {
                false
            }
        }
        NativeType::Int16 => {
            if let Ok(val) = canonical_str.parse::<u16>() {
                if is_negative_context {
                    val <= 32768
                } else {
                    val <= 32767
                }
            } else {
                false
            }
        }
        NativeType::Int | NativeType::Int32 => {
            if let Ok(val) = canonical_str.parse::<u32>() {
                if is_negative_context {
                    val <= 2147483648
                } else {
                    val <= 2147483647
                }
            } else {
                false
            }
        }
        NativeType::Int64 => {
            if let Ok(val) = canonical_str.parse::<u64>() {
                if is_negative_context {
                    val <= 9223372036854775808
                } else {
                    val <= 9223372036854775807
                }
            } else {
                false
            }
        }
        NativeType::Int128 => {
            if let Ok(val) = canonical_str.parse::<u128>() {
                if is_negative_context {
                    val <= 170141183460469231731687303715884105728
                } else {
                    val <= 170141183460469231731687303715884105727
                }
            } else {
                false
            }
        }
        NativeType::Uint8 => canonical_str.parse::<u8>().is_ok(),
        NativeType::Uint16 => canonical_str.parse::<u16>().is_ok(),
        NativeType::Uint32 => canonical_str.parse::<u32>().is_ok(),
        NativeType::Uint64 => canonical_str.parse::<u64>().is_ok(),
        NativeType::Uint128 => canonical_str.parse::<u128>().is_ok(),
        _ => false,
    }
}

fn get_conversion_target_native(name: &str) -> Option<NativeType> {
    match name {
        "to_int" => Some(NativeType::Int),
        "to_int8" => Some(NativeType::Int8),
        "to_int16" => Some(NativeType::Int16),
        "to_int32" => Some(NativeType::Int32),
        "to_int64" => Some(NativeType::Int64),
        "to_int128" => Some(NativeType::Int128),
        "to_uint8" => Some(NativeType::Uint8),
        "to_uint16" => Some(NativeType::Uint16),
        "to_uint32" => Some(NativeType::Uint32),
        "to_uint64" => Some(NativeType::Uint64),
        "to_uint128" => Some(NativeType::Uint128),
        "to_float" => Some(NativeType::Float),
        "to_float32" => Some(NativeType::Float32),
        "to_float64" => Some(NativeType::Float64),
        "to_string" => Some(NativeType::String),
        _ => None,
    }
}

fn native_type_name(nt: &NativeType) -> &'static str {
    match nt {
        NativeType::Int => "int",
        NativeType::Float => "float",
        NativeType::Bool => "bool",
        NativeType::String => "string",
        NativeType::Dynamic => "dynamic",
        NativeType::Int8 => "int8",
        NativeType::Int16 => "int16",
        NativeType::Int32 => "int32",
        NativeType::Int64 => "int64",
        NativeType::Int128 => "int128",
        NativeType::Uint8 => "uint8",
        NativeType::Uint16 => "uint16",
        NativeType::Uint32 => "uint32",
        NativeType::Uint64 => "uint64",
        NativeType::Uint128 => "uint128",
        NativeType::Float32 => "float32",
        NativeType::Float64 => "float64",
    }
}

fn is_valid_conversion(source_type: &SemanticType, target_native: &NativeType) -> bool {
    let source_native = match source_type {
        SemanticType::Native(nt) => nt,
        _ => return false, // composite types cannot be converted with to_tipo
    };

    if matches!(target_native, NativeType::String) {
        // to_string accepts only fixed numeric and dynamic
        return matches!(
            source_native,
            NativeType::Int
                | NativeType::Int8
                | NativeType::Int16
                | NativeType::Int32
                | NativeType::Int64
                | NativeType::Int128
                | NativeType::Uint8
                | NativeType::Uint16
                | NativeType::Uint32
                | NativeType::Uint64
                | NativeType::Uint128
                | NativeType::Float
                | NativeType::Float32
                | NativeType::Float64
                | NativeType::Dynamic
        );
    }

    // target is numeric (int*, uint*, float*)
    // source must be numeric or dynamic
    is_numeric_native(source_native)
}

#[cfg(test)]
#[allow(clippy::result_large_err)]
mod tests {
    use super::*;
    use crate::collaborators::lexer::lex_source;
    use crate::collaborators::parser::parse_tokens;
    use crate::data::compilation_dependency::*;

    fn analyze_src(
        source: &str,
        catalog: &CompilationCatalog,
    ) -> Result<SemanticProgram, CompileFailure> {
        let tokens = match lex_source(source) {
            Ok(t) => t,
            Err(_) => panic!("lex failed"),
        };
        let program = match parse_tokens(&tokens, source) {
            Ok(p) => p,
            Err(_) => panic!("parse failed"),
        };
        analyze_program(&program, catalog)
    }

    fn unwrap_err(res: Result<SemanticProgram, CompileFailure>) -> CompileFailure {
        match res {
            Ok(_) => panic!("expected CompileFailure, got Ok"),
            Err(e) => e,
        }
    }

    #[test]
    fn typed_binding_and_signatures() {
        let analyze_fn: Analyze = analyze_program;
        let bound: Analyze = ANALYZE_PROGRAM;
        assert_eq!(analyze_fn as usize, bound as usize);
    }

    #[test]
    fn semantic_program_ownership_independence() {
        let sem_prog = {
            let cat = CompilationCatalog {
                types: HashMap::new(),
                signatures: HashMap::new(),
            };
            let src = "public fn main() -> int { return 42; }".to_string();
            let tokens = match lex_source(&src) {
                Ok(t) => t,
                Err(_) => panic!("lex failed"),
            };
            let prog = match parse_tokens(&tokens, &src) {
                Ok(p) => p,
                Err(_) => panic!("parse failed"),
            };
            match analyze_program(&prog, &cat) {
                Ok(p) => p,
                Err(_) => panic!("analyze failed"),
            }
        };

        // Source, catalog, tokens, and AST dropped above; sem_prog is completely usable
        assert_eq!(sem_prog.functions.len(), 1);
        assert_eq!(sem_prog.entry_function.0, 0);
    }

    // --- ResolutionFailures ---

    #[test]
    fn resolution_imported_symbol_not_found() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "import math::sqrt; public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Resolution(
                ResolutionFailure::ImportedSymbolNotFound { module, name },
            )) => {
                assert_eq!(&*module, "math");
                assert_eq!(&*name, "sqrt");
            }
            _ => panic!("expected ImportedSymbolNotFound"),
        }
    }

    #[test]
    fn resolution_unknown_type() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main(UnknownType x) -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Resolution(
                ResolutionFailure::UnknownType { name },
            )) => {
                assert_eq!(&*name, "UnknownType");
            }
            _ => panic!("expected UnknownType"),
        }
    }

    #[test]
    fn resolution_unknown_value_symbol() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return missing_var; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Resolution(
                ResolutionFailure::UnknownValueSymbol { name },
            )) => {
                assert_eq!(&*name, "missing_var");
            }
            _ => panic!("expected UnknownValueSymbol"),
        }
    }

    #[test]
    fn resolution_unknown_signature() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main(ext::MissingSig dep) -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Resolution(
                ResolutionFailure::UnknownSignature(sym),
            )) => {
                assert_eq!(sym.module, "ext");
                assert_eq!(sym.name, "MissingSig");
            }
            _ => panic!("expected UnknownSignature"),
        }
    }

    // --- DeclarationFailures ---

    #[test]
    fn declaration_type_name_collision() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src =
            "struct Point { int x; } struct Point { int y; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::TypeNameCollision { name },
            )) => {
                assert_eq!(&*name, "Point");
            }
            _ => panic!("expected TypeNameCollision"),
        }
    }

    #[test]
    fn declaration_duplicate_function() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "private fn helper() -> int { return 1; } private fn helper() -> int { return 2; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::DuplicateFunction { name },
            )) => {
                assert_eq!(&*name, "helper");
            }
            _ => panic!("expected DuplicateFunction"),
        }
    }

    #[test]
    fn declaration_duplicate_field() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct Worker { int id; string id; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::DuplicateField { name },
            )) => {
                assert_eq!(&*name, "id");
            }
            _ => panic!("expected DuplicateField"),
        }
    }

    #[test]
    fn declaration_duplicate_variant() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum Status { Active, Inactive, Active } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::DuplicateVariant { name },
            )) => {
                assert_eq!(&*name, "Active");
            }
            _ => panic!("expected DuplicateVariant"),
        }
    }

    #[test]
    fn declaration_binding_name_collision() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main(int val) -> int { let int val = 10; return val; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::BindingNameCollision { name },
            )) => {
                assert_eq!(&*name, "val");
            }
            _ => panic!("expected BindingNameCollision"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_type() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct bad_struct { int x; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::Type => {}
                _ => panic!("expected Type role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_field() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct Point { int BadField; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::Field => {}
                _ => panic!("expected Field role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn declaration_recursive_type_cycle_direct() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct Node { Node next; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::RecursiveTypeCycle,
            )) => {}
            _ => panic!("expected RecursiveTypeCycle"),
        }
    }

    #[test]
    fn declaration_recursive_type_cycle_indirect() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct A { B b; } struct B { C c; } struct C { A a; } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::RecursiveTypeCycle,
            )) => {}
            _ => panic!("expected RecursiveTypeCycle"),
        }
    }

    // --- TypeCheckingFailures ---

    #[test]
    fn type_checking_binding_initialization() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { let int x = true; return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::BindingInitialization { expected, actual },
            )) => match (&expected, &actual) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Int),
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                ) => {}
                _ => panic!("expected int and bool descriptors"),
            },
            _ => panic!("expected BindingInitialization"),
        }
    }

    #[test]
    fn type_checking_function_result() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return true; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::FunctionResult { expected, actual },
            )) => match (&expected, &actual) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Int),
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                ) => {}
                _ => panic!("expected int and bool descriptors"),
            },
            _ => panic!("expected FunctionResult"),
        }
    }

    #[test]
    fn type_checking_numeric_literal_not_representable() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int8 { return 500; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::NumericLiteralNotRepresentable { expected },
            )) => match expected {
                SemanticTypeDescriptor::Native(NativeType::Int8) => {}
                _ => panic!("expected Int8 descriptor"),
            },
            _ => panic!("expected NumericLiteralNotRepresentable"),
        }
    }

    #[test]
    fn type_checking_unary_operator() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> bool { return !10; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::UnaryOperator { operator, operand },
            )) => {
                match operator {
                    UnaryOperator::Not => {}
                    _ => panic!("expected Not"),
                }
                match operand {
                    SemanticTypeDescriptor::Native(NativeType::Int) => {}
                    _ => panic!("expected Int descriptor"),
                }
            }
            _ => panic!("expected UnaryOperator"),
        }
    }

    #[test]
    fn type_checking_arithmetic_operator() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return 10 + true; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::ArithmeticOperator {
                    operator,
                    left,
                    right,
                },
            )) => {
                match operator {
                    BinaryOperator::Add => {}
                    _ => panic!("expected Add"),
                }
                match (&left, &right) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int),
                        SemanticTypeDescriptor::Native(NativeType::Bool),
                    ) => {}
                    _ => panic!("expected Int and Bool"),
                }
            }
            _ => panic!("expected ArithmeticOperator"),
        }
    }

    #[test]
    fn type_checking_logical_operator() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> bool { return true && 1; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::LogicalOperator {
                    operator,
                    left,
                    right,
                },
            )) => {
                match operator {
                    BinaryOperator::And => {}
                    _ => panic!("expected And"),
                }
                match (&left, &right) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Bool),
                        SemanticTypeDescriptor::Native(NativeType::Int),
                    ) => {}
                    _ => panic!("expected Bool and Int"),
                }
            }
            _ => panic!("expected LogicalOperator"),
        }
    }

    #[test]
    fn type_checking_comparison() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> bool { return 10 == true; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::Comparison {
                    operator,
                    left,
                    right,
                },
            )) => {
                match operator {
                    BinaryOperator::Equal => {}
                    _ => panic!("expected Equal"),
                }
                match (&left, &right) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int),
                        SemanticTypeDescriptor::Native(NativeType::Bool),
                    ) => {}
                    _ => panic!("expected Int and Bool"),
                }
            }
            _ => panic!("expected Comparison"),
        }
    }

    #[test]
    fn type_checking_invalid_conversion() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return to_int(true); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                TypeCheckingFailure::InvalidConversion { source, target },
            )) => match (&source, &target) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                    SemanticTypeDescriptor::Native(NativeType::Int),
                ) => {}
                _ => panic!("expected Bool and Int"),
            },
            _ => panic!("expected InvalidConversion"),
        }
    }

    // --- CallFailures ---

    #[test]
    fn call_function_not_found() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return nonexistent(1); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(
                CallFailure::FunctionNotFound { name },
            )) => {
                assert_eq!(&*name, "nonexistent");
            }
            _ => panic!("expected FunctionNotFound"),
        }
    }

    #[test]
    fn call_ambiguous_target() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "ops".to_string(),
                name: "calc".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::Int)],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "import ops::calc; private fn calc(int x) -> int { return x; } public fn main() -> int { return calc(10); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(CallFailure::AmbiguousTarget {
                name,
            })) => {
                assert_eq!(&*name, "calc");
            }
            _ => panic!("expected AmbiguousTarget"),
        }
    }

    #[test]
    fn call_arity_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "private fn add(int a, int b) -> int { return a + b; } public fn main() -> int { return add(1); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(CallFailure::ArityMismatch {
                expected,
                actual,
            })) => {
                assert_eq!(expected, 2);
                assert_eq!(actual, 1);
            }
            _ => panic!("expected ArityMismatch"),
        }
    }

    #[test]
    fn call_argument_kind_mismatch() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "ext".to_string(),
                name: "Log".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::String)],
                result_type: CatalogTypeRef::Bool,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "private fn run(ext::Log logger) -> int { return 0; } public fn main() -> int { return run(10); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(
                CallFailure::ArgumentKindMismatch {
                    position,
                    expected,
                    actual,
                },
            )) => {
                assert_eq!(position, 0);
                match (expected, actual) {
                    (SemanticArgumentKind::SignatureDependency, SemanticArgumentKind::Value) => {}
                    _ => panic!("expected SignatureDependency and Value"),
                }
            }
            _ => panic!("expected ArgumentKindMismatch"),
        }
    }

    #[test]
    fn call_argument_type_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "private fn add(int a, int b) -> int { return a + b; } public fn main() -> int { return add(1, true); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(
                CallFailure::ArgumentTypeMismatch {
                    position,
                    expected,
                    actual,
                },
            )) => {
                assert_eq!(position, 1);
                match (&expected, &actual) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int),
                        SemanticTypeDescriptor::Native(NativeType::Bool),
                    ) => {}
                    _ => panic!("expected Int and Bool"),
                }
            }
            _ => panic!("expected ArgumentTypeMismatch"),
        }
    }

    #[test]
    fn call_signature_dependency_mismatch() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "ext".to_string(),
                name: "LogA".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Bool,
            },
        );
        signatures.insert(
            SignatureSymbol {
                module: "ext".to_string(),
                name: "LogB".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Bool,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "private fn helper(ext::LogA logger) -> int { return 0; } public fn main(ext::LogB other_logger) -> int { return helper(other_logger); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(
                CallFailure::SignatureDependencyMismatch {
                    position,
                    expected,
                    actual,
                },
            )) => {
                assert_eq!(position, 0);
                assert_eq!(expected.name, "LogA");
                assert_eq!(actual.name, "LogB");
            }
            _ => panic!("expected SignatureDependencyMismatch"),
        }
    }

    #[test]
    fn call_function_call_cycle_direct() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { return main(); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(CallFailure::FunctionCallCycle)) => {
            }
            _ => panic!("expected FunctionCallCycle"),
        }
    }

    #[test]
    fn call_function_call_cycle_indirect() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "private fn foo() -> int { return bar(); } private fn bar() -> int { return foo(); } public fn main() -> int { return foo(); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Call(CallFailure::FunctionCallCycle)) => {
            }
            _ => panic!("expected FunctionCallCycle"),
        }
    }

    // --- CompositeFailures ---

    #[test]
    fn composite_expected_struct() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main() -> int { let int s = int { x: 1 }; return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::ExpectedStruct { actual },
            )) => match actual {
                SemanticTypeDescriptor::Native(NativeType::Int) => {}
                _ => panic!("expected Int"),
            },
            _ => panic!("expected ExpectedStruct"),
        }
    }

    #[test]
    fn composite_field_access_type() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main(int val) -> int { return val.field; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::FieldAccessType { actual },
            )) => match actual {
                SemanticTypeDescriptor::Native(NativeType::Int) => {}
                _ => panic!("expected Int"),
            },
            _ => panic!("expected FieldAccessType"),
        }
    }

    #[test]
    fn composite_field_not_found() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct Point { int x; } public fn main(Point p) -> int { return p.y; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::FieldNotFound { field },
            )) => {
                assert_eq!(&*field, "y");
            }
            _ => panic!("expected FieldNotFound"),
        }
    }

    #[test]
    fn composite_missing_field() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src =
            "struct Point { int x; int y; } public fn main() -> Point { return Point { x: 10 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::MissingField { field },
            )) => {
                assert_eq!(&*field, "y");
            }
            _ => panic!("expected MissingField"),
        }
    }

    #[test]
    fn composite_duplicate_field_initializer() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src =
            "struct Point { int x; } public fn main() -> Point { return Point { x: 10, x: 20 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::DuplicateFieldInitializer { field },
            )) => {
                assert_eq!(&*field, "x");
            }
            _ => panic!("expected DuplicateFieldInitializer"),
        }
    }

    #[test]
    fn composite_field_type_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "struct Point { int x; } public fn main() -> Point { return Point { x: true }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::FieldTypeMismatch {
                    field,
                    expected,
                    actual,
                },
            )) => {
                assert_eq!(&*field, "x");
                match (&expected, &actual) {
                    (
                        SemanticTypeDescriptor::Native(NativeType::Int),
                        SemanticTypeDescriptor::Native(NativeType::Bool),
                    ) => {}
                    _ => panic!("expected Int and Bool"),
                }
            }
            _ => panic!("expected FieldTypeMismatch"),
        }
    }

    #[test]
    fn composite_variant_not_found() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Ready } public fn main() -> State { return State::Done; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::VariantNotFound { variant },
            )) => {
                assert_eq!(&*variant, "Done");
            }
            _ => panic!("expected VariantNotFound"),
        }
    }

    #[test]
    fn composite_variant_payload_shape_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Ready, Payload(int) } public fn main() -> State { return State::Ready(10); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::VariantPayloadShapeMismatch { expected, actual },
            )) => match (expected, actual) {
                (EnumPayloadShape::Simple, EnumPayloadShape::Associated) => {}
                _ => panic!("expected Simple and Associated"),
            },
            _ => panic!("expected VariantPayloadShapeMismatch"),
        }
    }

    #[test]
    fn composite_associated_payload_type_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Payload(int) } public fn main() -> State { return State::Payload(true); }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Composite(
                CompositeFailure::AssociatedPayloadTypeMismatch { expected, actual },
            )) => match (&expected, &actual) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Int),
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                ) => {}
                _ => panic!("expected Int and Bool"),
            },
            _ => panic!("expected AssociatedPayloadTypeMismatch"),
        }
    }

    // --- WhenFailures ---

    #[test]
    fn when_subject_not_enum() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main(int x) -> int { return when x { State::A => 1 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(WhenFailure::SubjectNotEnum {
                actual,
            })) => match actual {
                SemanticTypeDescriptor::Native(NativeType::Int) => {}
                _ => panic!("expected Int"),
            },
            _ => panic!("expected SubjectNotEnum"),
        }
    }

    #[test]
    fn when_pattern_enum_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src =
            "enum A { V } enum B { V } public fn main(A a) -> int { return when a { B::V => 1 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(
                WhenFailure::PatternEnumMismatch { expected, actual },
            )) => match (&expected, &actual) {
                (SemanticTypeDescriptor::Local(e), SemanticTypeDescriptor::Local(a)) => {
                    assert_eq!(&**e, "A");
                    assert_eq!(&**a, "B");
                }
                _ => panic!("expected local enum descriptors"),
            },
            _ => panic!("expected PatternEnumMismatch"),
        }
    }

    #[test]
    fn when_duplicate_variant_correspondence() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Ready, Done } public fn main(State s) -> int { return when s { State::Ready => 1, State::Ready => 2 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(
                WhenFailure::DuplicateVariantCorrespondence { variant },
            )) => {
                assert_eq!(&*variant, "Ready");
            }
            _ => panic!("expected DuplicateVariantCorrespondence"),
        }
    }

    #[test]
    fn when_non_exhaustive_order() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Alpha, Beta, Gamma } public fn main(State s) -> int { return when s { State::Beta => 2 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(WhenFailure::NonExhaustive {
                missing,
            })) => {
                assert_eq!(missing.len(), 2);
                assert_eq!(&*missing[0], "Alpha");
                assert_eq!(&*missing[1], "Gamma");
            }
            _ => panic!("expected NonExhaustive with Alpha and Gamma in order"),
        }
    }

    #[test]
    fn when_extraction_type_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Payload(int) } public fn main(State s) -> int { return when s { State::Payload(bool b) => 1 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(
                WhenFailure::ExtractionTypeMismatch { expected, actual },
            )) => match (&expected, &actual) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Int),
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                ) => {}
                _ => panic!("expected Int and Bool"),
            },
            _ => panic!("expected ExtractionTypeMismatch"),
        }
    }

    #[test]
    fn when_branch_result_type_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { A, B } public fn main(State s) -> int { return when s { State::A => 1, State::B => true }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(
                WhenFailure::BranchResultTypeMismatch { expected, actual },
            )) => match (&expected, &actual) {
                (
                    SemanticTypeDescriptor::Native(NativeType::Int),
                    SemanticTypeDescriptor::Native(NativeType::Bool),
                ) => {}
                _ => panic!("expected Int and Bool"),
            },
            _ => panic!("expected BranchResultTypeMismatch"),
        }
    }

    // --- SignatureMismatch ---

    #[test]
    fn signature_mismatch_function_name() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "Calculate".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn run() -> int : svc::Calculate { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                signature,
                mismatch,
            }) => {
                assert_eq!(signature.module, "svc");
                assert_eq!(signature.name, "Calculate");
                match mismatch {
                    SignatureMismatchKind::FunctionName => {}
                    _ => panic!("expected FunctionName mismatch"),
                }
            }
            _ => panic!("expected SignatureMismatch"),
        }
    }

    #[test]
    fn signature_mismatch_parameter_count() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "calc".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::Int)],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn calc(int a, int b) -> int : svc::calc { return a + b; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                mismatch, ..
            }) => match mismatch {
                SignatureMismatchKind::ParameterCount { expected, actual } => {
                    assert_eq!(expected, 1);
                    assert_eq!(actual, 2);
                }
                _ => panic!("expected ParameterCount mismatch"),
            },
            _ => panic!("expected SignatureMismatch"),
        }
    }

    #[test]
    fn signature_mismatch_value_parameter_type() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "calc".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::Int)],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn calc(bool flag) -> int : svc::calc { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                mismatch, ..
            }) => match mismatch {
                SignatureMismatchKind::ValueParameterType {
                    position,
                    expected,
                    actual,
                } => {
                    assert_eq!(position, 0);
                    match (&expected, &actual) {
                        (
                            SemanticTypeDescriptor::Native(NativeType::Int),
                            SemanticTypeDescriptor::Native(NativeType::Bool),
                        ) => {}
                        _ => panic!("expected Int and Bool"),
                    }
                }
                _ => panic!("expected ValueParameterType mismatch"),
            },
            _ => panic!("expected SignatureMismatch"),
        }
    }

    #[test]
    fn signature_mismatch_result_type() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "calc".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn calc() -> bool : svc::calc { return true; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                mismatch, ..
            }) => match mismatch {
                SignatureMismatchKind::ResultType { expected, actual } => {
                    match (&expected, &actual) {
                        (
                            SemanticTypeDescriptor::Native(NativeType::Int),
                            SemanticTypeDescriptor::Native(NativeType::Bool),
                        ) => {}
                        _ => panic!("expected Int and Bool"),
                    }
                }
                _ => panic!("expected ResultType mismatch"),
            },
            _ => panic!("expected SignatureMismatch"),
        }
    }

    // --- Success Integral Test ---

    #[test]
    fn semantic_success_integral_pipeline_and_conversions() {
        let mut types = HashMap::new();
        types.insert(
            TypeSymbol {
                module: "data".to_string(),
                name: "ExternalUser".to_string(),
            },
            CatalogType::Struct {
                fields: alloc::vec![CatalogField {
                    name: "id".to_string(),
                    type_ref: CatalogTypeRef::Int64,
                }],
            },
        );

        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "io".to_string(),
                name: "Log".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::String)],
                result_type: CatalogTypeRef::Bool,
            },
        );

        let cat = CompilationCatalog { types, signatures };

        let src = r#"
            import data::ExternalUser as ExtUser;
            import io::Log;

            struct LocalWorker {
                int id;
                string name;
            }

            enum Status {
                Active,
                WithId(int),
                Detailed { int code; string message; }
            }

            private fn helper(int x) -> int {
                return x + 1;
            }

            public fn main(ExtUser user, io::Log logger) -> int {
                let LocalWorker worker = LocalWorker { id: 10, name: "Alice" };
                let Status st = Status::Detailed { code: 200, message: "OK" };

                let int status_code = when st {
                    Status::Active => 0,
                    Status::WithId(int id_val) => id_val,
                    Status::Detailed { code: int code; message: string message; } => code
                };

                let int converted = 100 |> to_int64 |> to_int;
                let int piped = worker.id |> helper;

                return piped + status_code;
            }
        "#;

        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("integral analysis must succeed"),
        };
        assert_eq!(sem_prog.functions.len(), 2);
        assert_eq!(sem_prog.entry_function.0, 1);

        let main_fn = &sem_prog.functions[1];
        assert_eq!(main_fn.parameters.len(), 2);
        match &main_fn.parameters[0] {
            SemanticParameter::Value(bid) => assert_eq!(bid.0, 0),
            _ => panic!("expected Value parameter"),
        }
        match &main_fn.parameters[1] {
            SemanticParameter::SignatureDependency(sbid) => assert_eq!(sbid.0, 0),
            _ => panic!("expected SignatureDependency parameter"),
        }

        // Verify pipeline lowered to Conversion / Call without Pipeline identity
        match &main_fn.body.statements[3] {
            SemanticStatement::Bind { value, .. } => match &value.kind {
                SemanticExpressionKind::Conversion { operand } => match &operand.kind {
                    SemanticExpressionKind::Conversion { .. } => {}
                    _ => panic!("expected inner Conversion"),
                },
                _ => panic!("expected outer Conversion for pipeline of conversions"),
            },
            _ => panic!("expected Bind"),
        }

        match &main_fn.body.statements[4] {
            SemanticStatement::Bind { value, .. } => match &value.kind {
                SemanticExpressionKind::Call(call) => match &call.target {
                    SemanticCallTarget::Internal(fid) => assert_eq!(fid.0, 0),
                    _ => panic!("expected Internal call target for piped helper"),
                },
                _ => panic!("expected Call for pipeline of function"),
            },
            _ => panic!("expected Bind"),
        }
    }

    #[test]
    fn arbitrary_large_integer_dynamic() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            public fn main() -> dynamic {
                let dynamic big = 34028236692093846346337460743176821145600000000000;
                return big;
            }
        "#;
        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("large integer for dynamic should succeed"),
        };
        let main_fn = &sem_prog.functions[0];
        match &main_fn.body.statements[0] {
            SemanticStatement::Bind { value, .. } => match &value.kind {
                SemanticExpressionKind::Literal(SemanticLiteral::Integer(s)) => {
                    assert_eq!(s, "34028236692093846346337460743176821145600000000000");
                }
                _ => panic!("expected Integer literal"),
            },
            _ => panic!("expected Bind"),
        }
    }

    #[test]
    fn field_initializers_reordered() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            struct Worker {
                int id;
                string name;
            }

            public fn main() -> Worker {
                return Worker { name: "Alice", id: 42 };
            }
        "#;
        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("reordered field initializers should succeed"),
        };
        let main_fn = &sem_prog.functions[0];
        match &main_fn.body.result.kind {
            SemanticExpressionKind::StructConstruction { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].field.0, 1); // name is field 1 (source first)
                assert_eq!(fields[1].field.0, 0); // id is field 0 (source second)
            }
            _ => panic!("expected StructConstruction"),
        }
    }

    #[test]
    fn structured_enum_fields_reordered() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Message {
                Data { int code; string payload; }
            }

            public fn main() -> Message {
                return Message::Data { payload: "hello", code: 200 };
            }
        "#;
        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("reordered enum fields should succeed"),
        };
        let main_fn = &sem_prog.functions[0];
        match &main_fn.body.result.kind {
            SemanticExpressionKind::EnumConstruction { payload, .. } => match payload {
                SemanticEnumPayload::Structured { fields } => {
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].field.0, 1); // payload is field 1 (source first)
                    assert_eq!(fields[1].field.0, 0); // code is field 0 (source second)
                }
                _ => panic!("expected Structured payload"),
            },
            _ => panic!("expected EnumConstruction"),
        }
    }

    #[test]
    fn when_structured_fields_reordered() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = r#"
            enum Message {
                Data { int code; string payload; }
            }

            public fn main(Message msg) -> int {
                return when msg {
                    Message::Data { payload: string p; code: int c; } => c
                };
            }
        "#;
        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("reordered when structured fields should succeed"),
        };
        let main_fn = &sem_prog.functions[0];
        match &main_fn.body.result.kind {
            SemanticExpressionKind::When(when) => {
                assert_eq!(when.branches.len(), 1);
                match &when.branches[0].extraction {
                    SemanticVariantExtraction::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        assert_eq!(fields[0].field.0, 0); // code is field 0
                        assert_eq!(fields[1].field.0, 1); // payload is field 1
                    }
                    _ => panic!("expected Structured extraction"),
                }
            }
            _ => panic!("expected When expression"),
        }
    }

    #[test]
    fn signature_dependency_forwarding_and_direct_call() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "io".to_string(),
                name: "Log".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::String)],
                result_type: CatalogTypeRef::Bool,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = r#"
            import io::Log;

            private fn forward(io::Log l) -> bool {
                return l("forwarded");
            }

            public fn main(io::Log root_logger) -> bool {
                let bool r1 = forward(root_logger);
                let bool r2 = Log("direct");
                return r1 && r2;
            }
        "#;
        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("signature forwarding and direct call should succeed"),
        };
        assert_eq!(sem_prog.signatures.len(), 1);
        assert_eq!(sem_prog.functions.len(), 2);

        let fwd_fn = &sem_prog.functions[0];
        match &fwd_fn.body.result.kind {
            SemanticExpressionKind::Call(call) => match &call.target {
                SemanticCallTarget::SignatureDependency(sbid) => assert_eq!(sbid.0, 0),
                _ => panic!("expected SignatureDependency target"),
            },
            _ => panic!("expected Call"),
        }

        let main_fn = &sem_prog.functions[1];
        match &main_fn.body.statements[1] {
            SemanticStatement::Bind { value, .. } => match &value.kind {
                SemanticExpressionKind::Call(call) => match &call.target {
                    SemanticCallTarget::DirectSignature(sid) => assert_eq!(sid.0, 0),
                    _ => panic!("expected DirectSignature target"),
                },
                _ => panic!("expected Call"),
            },
            _ => panic!("expected Bind"),
        }
    }

    #[test]
    fn signature_transitive_dependency_materialization_ids() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "dep".to_string(),
                name: "B".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::String)],
                result_type: CatalogTypeRef::Bool,
            },
        );
        signatures.insert(
            SignatureSymbol {
                module: "dep".to_string(),
                name: "A".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::SignatureDependency(
                    SignatureSymbol {
                        module: "dep".to_string(),
                        name: "B".to_string(),
                    }
                )],
                result_type: CatalogTypeRef::Int,
            },
        );

        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = r#"
            import dep::A;

            public fn main() -> int {
                return 0;
            }
        "#;
        let sem_prog = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("signature transitive materialization should succeed"),
        };

        assert_eq!(sem_prog.signatures.len(), 2);
        let a_idx = sem_prog
            .signatures
            .iter()
            .position(|s| s.symbol.module == "dep" && s.symbol.name == "A")
            .expect("A must exist");
        let b_idx = sem_prog
            .signatures
            .iter()
            .position(|s| s.symbol.module == "dep" && s.symbol.name == "B")
            .expect("B must exist");

        assert_ne!(a_idx, b_idx);
        assert_eq!(sem_prog.signatures[a_idx].symbol.module, "dep");
        assert_eq!(sem_prog.signatures[a_idx].symbol.name, "A");
        assert_eq!(sem_prog.signatures[b_idx].symbol.module, "dep");
        assert_eq!(sem_prog.signatures[b_idx].symbol.name, "B");

        match &sem_prog.signatures[a_idx].parameters[0] {
            SemanticSignatureParameter::SignatureDependency(sid) => {
                assert_eq!(sid.0, b_idx);
            }
            _ => panic!("expected SignatureDependency for A parameter"),
        }
    }

    #[test]
    fn call_graph_internal_vs_direct_sig_ambiguity_no_premature_cycle() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "ops".to_string(),
                name: "work".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = r#"
            import ops::work;

            private fn work() -> int {
                return work();
            }

            public fn main() -> int {
                return 0;
            }
        "#;
        let res = analyze_src(src, &cat);
        match res {
            Err(CompileFailure {
                kind:
                    CompileFailureKind::Semantic(SemanticFailure::Call(CallFailure::AmbiguousTarget {
                        name,
                    })),
                ..
            }) => {
                assert_eq!(&*name, "work");
            }
            other => panic!("expected AmbiguousTarget, got {:?}", other.is_ok()),
        }
    }

    #[test]
    fn call_multiple_direct_signatures_same_local_name() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "a".to_string(),
                name: "save".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        signatures.insert(
            SignatureSymbol {
                module: "b".to_string(),
                name: "save".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = r#"
            import a::save;
            import b::save;

            public fn main() -> int {
                return save();
            }
        "#;
        let res = analyze_src(src, &cat);
        match res {
            Err(CompileFailure {
                kind:
                    CompileFailureKind::Semantic(SemanticFailure::Call(CallFailure::AmbiguousTarget {
                        name,
                    })),
                ..
            }) => {
                assert_eq!(&*name, "save");
            }
            other => panic!("expected AmbiguousTarget, got {:?}", other.is_ok()),
        }
    }

    #[test]
    fn order_comparison_string_forbidden() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src1 = r#"
            public fn main() -> bool {
                return "a" < "b";
            }
        "#;
        assert!(matches!(
            analyze_src(src1, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::Comparison { .. }
                )),
                ..
            })
        ));

        let src2 = r#"
            public fn main() -> bool {
                return "a" >= "b";
            }
        "#;
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::Comparison { .. }
                )),
                ..
            })
        ));

        let src_valid = r#"
            public fn main() -> bool {
                return ("a" == "a") && ("a" != "b");
            }
        "#;
        assert!(analyze_src(src_valid, &cat).is_ok());
    }

    #[test]
    fn equality_comparison_composite_dynamic_forbidden() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // Direct struct with dynamic field
        let src1 = r#"
            struct A {
                dynamic val;
            }
            public fn main(A a, A b) -> bool {
                return a == b;
            }
        "#;
        assert!(matches!(
            analyze_src(src1, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::Comparison { .. }
                )),
                ..
            })
        ));

        // Transitive struct with dynamic field
        let src2 = r#"
            struct Inner {
                dynamic val;
            }
            struct Outer {
                Inner inner;
            }
            public fn main(Outer a, Outer b) -> bool {
                return a == b;
            }
        "#;
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::Comparison { .. }
                )),
                ..
            })
        ));

        // Enum with dynamic payload
        let src3 = r#"
            enum State {
                Payload(dynamic)
            }
            public fn main(State a, State b) -> bool {
                return a == b;
            }
        "#;
        assert!(matches!(
            analyze_src(src3, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::Comparison { .. }
                )),
                ..
            })
        ));

        // Valid struct equality
        let src_valid = r#"
            struct ValidA {
                int id;
                string name;
            }
            public fn main(ValidA a, ValidA b) -> bool {
                return (a == b) && (a != b);
            }
        "#;
        assert!(analyze_src(src_valid, &cat).is_ok());
    }

    #[test]
    fn numeric_literal_signed_positive_max_and_negative_min() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // let int8 x = 127 -> success
        let src1 = "public fn main() -> int8 { let int8 x = 127; return x; }";
        assert!(analyze_src(src1, &cat).is_ok());

        // let int8 x = 128 -> NumericLiteralNotRepresentable
        let src2 = "public fn main() -> int8 { let int8 x = 128; return x; }";
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::NumericLiteralNotRepresentable { .. }
                )),
                ..
            })
        ));

        // let int8 x = -128 -> success
        let src3 = "public fn main() -> int8 { let int8 x = -128; return x; }";
        assert!(analyze_src(src3, &cat).is_ok());

        // let int8 x = -129 -> NumericLiteralNotRepresentable
        let src4 = "public fn main() -> int8 { let int8 x = -129; return x; }";
        assert!(matches!(
            analyze_src(src4, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::NumericLiteralNotRepresentable { .. }
                )),
                ..
            })
        ));

        // let int64 x = -100 -> success
        let src5 = "public fn main() -> int64 { let int64 x = -100; return x; }";
        assert!(analyze_src(src5, &cat).is_ok());

        // positive int 2147483648 without larger context -> NumericLiteralNotRepresentable
        let src6 = "public fn main() -> int { let int x = 2147483648; return x; }";
        assert!(matches!(
            analyze_src(src6, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::NumericLiteralNotRepresentable { .. }
                )),
                ..
            })
        ));

        // let uint8 x = -1 -> UnaryOperator error
        let src7 = "public fn main() -> uint8 { let uint8 x = -1; return x; }";
        assert!(matches!(
            analyze_src(src7, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::UnaryOperator { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn floating_literal_representability_and_overflow() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src1 = "public fn main() -> float32 { let float32 x = 1.5; return x; }";
        assert!(analyze_src(src1, &cat).is_ok());

        let src2 = "public fn main() -> float32 { let float32 x = 1e100; return x; }";
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::NumericLiteralNotRepresentable { .. }
                )),
                ..
            })
        ));

        let src3 = "public fn main() -> float { let float x = 1e400; return x; }";
        assert!(matches!(
            analyze_src(src3, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::NumericLiteralNotRepresentable { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn dynamic_context_binary_arithmetic_same_type() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // Binary arithmetic under dynamic context gets type_id: dynamic
        let src1 = "public fn main(int8 a, int8 b) -> dynamic { return a + b; }";
        let sem1 = match analyze_src(src1, &cat) {
            Ok(p) => p,
            Err(_) => panic!("binary under dynamic context should succeed"),
        };
        let dyn_id = sem1
            .types
            .iter()
            .position(|t| matches!(t, SemanticType::Native(NativeType::Dynamic)))
            .unwrap();
        assert_eq!(sem1.functions[0].body.result.type_id.0, dyn_id);

        // Heterogeneous fixed types fail under dynamic context
        let src2 = "public fn main(int32 a, int64 b) -> dynamic { return a + b; }";
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // Unary under dynamic context gets type_id: dynamic
        let src3 = "public fn main(int8 a) -> dynamic { return -a; }";
        let sem3 = match analyze_src(src3, &cat) {
            Ok(p) => p,
            Err(_) => panic!("unary under dynamic context should succeed"),
        };
        assert_eq!(sem3.functions[0].body.result.type_id.0, dyn_id);

        // Function boundary: calculate returns int8, returned to dynamic in caller
        let src4 = r#"
            private fn calculate(int8 a, int8 b) -> int8 {
                return a + b;
            }
            public fn main(int8 a, int8 b) -> dynamic {
                return calculate(a, b);
            }
        "#;
        assert!(analyze_src(src4, &cat).is_ok());
    }

    #[test]
    fn contextual_numeric_symmetry() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src1 = "public fn main(int64 x) -> bool { return x == 1; }";
        assert!(analyze_src(src1, &cat).is_ok());

        let src2 = "public fn main(int64 x) -> bool { return 1 == x; }";
        assert!(analyze_src(src2, &cat).is_ok());

        let src3 = "public fn main(float32 x) -> float32 { return x + 1.5; }";
        assert!(analyze_src(src3, &cat).is_ok());

        let src4 = "public fn main(float32 x) -> float32 { return 1.5 + x; }";
        assert!(analyze_src(src4, &cat).is_ok());
    }

    #[test]
    fn dynamic_rejects_nonnumeric() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // let dynamic x = true -> BindingInitialization
        let src1 = "public fn main() -> int { let dynamic x = true; return 0; }";
        assert!(matches!(
            analyze_src(src1, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::BindingInitialization { .. }
                )),
                ..
            })
        ));

        // let dynamic x = "hello" -> BindingInitialization
        let src2 = "public fn main() -> int { let dynamic x = \"hello\"; return 0; }";
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::BindingInitialization { .. }
                )),
                ..
            })
        ));

        // fn -> dynamic { return true; } -> FunctionResult
        let src3 = "public fn main() -> dynamic { return true; }";
        assert!(matches!(
            analyze_src(src3, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::FunctionResult { .. }
                )),
                ..
            })
        ));

        // fn -> dynamic { return "hello"; } -> FunctionResult
        let src4 = "public fn main() -> dynamic { return \"hello\"; }";
        assert!(matches!(
            analyze_src(src4, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::FunctionResult { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn fixed_numeric_value_to_dynamic() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // int8 binding -> dynamic let
        let src1 = "public fn main(int8 source) -> dynamic { let dynamic x = source; return x; }";
        assert!(analyze_src(src1, &cat).is_ok());

        // int8 function result -> dynamic return
        let src2 = "public fn main(int8 value) -> dynamic { return value; }";
        assert!(analyze_src(src2, &cat).is_ok());

        // float32 binding -> dynamic let
        let src3 =
            "public fn main(float32 source) -> dynamic { let dynamic x = source; return x; }";
        assert!(analyze_src(src3, &cat).is_ok());
    }

    #[test]
    fn dynamic_call_argument() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src1 = r#"
            private fn consume(dynamic value) -> dynamic {
                return value;
            }
            public fn main(int8 value) -> dynamic {
                return consume(value);
            }
        "#;
        assert!(analyze_src(src1, &cat).is_ok());

        let src2 = r#"
            private fn consume(dynamic value) -> dynamic {
                return value;
            }
            public fn main(bool value) -> dynamic {
                return consume(value);
            }
        "#;
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Call(
                    CallFailure::ArgumentTypeMismatch { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn dynamic_pipeline_argument() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src1 = r#"
            private fn consume(dynamic value) -> dynamic {
                return value;
            }
            public fn main(int8 value) -> dynamic {
                return value |> consume;
            }
        "#;
        assert!(analyze_src(src1, &cat).is_ok());
    }

    #[test]
    fn dynamic_composite_fields() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // dynamic struct field <- int8 -> success
        let src1 = r#"
            struct Holder {
                dynamic value;
            }
            public fn main(int8 source) -> Holder {
                return Holder { value: source };
            }
        "#;
        assert!(analyze_src(src1, &cat).is_ok());

        // dynamic struct field <- bool -> FieldTypeMismatch
        let src2 = r#"
            struct Holder {
                dynamic value;
            }
            public fn main() -> Holder {
                return Holder { value: true };
            }
        "#;
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                    CompositeFailure::FieldTypeMismatch { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn dynamic_enum_payload() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // dynamic associated payload <- int8 -> success
        let src1 = r#"
            enum Result {
                Value(dynamic)
            }
            public fn main(int8 source) -> Result {
                return Result::Value(source);
            }
        "#;
        assert!(analyze_src(src1, &cat).is_ok());

        // dynamic associated payload <- bool -> AssociatedPayloadTypeMismatch
        let src2 = r#"
            enum Result {
                Value(dynamic)
            }
            public fn main() -> Result {
                return Result::Value(true);
            }
        "#;
        assert!(matches!(
            analyze_src(src2, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                    CompositeFailure::AssociatedPayloadTypeMismatch { .. }
                )),
                ..
            })
        ));

        // dynamic structured payload field <- int8 -> success
        let src3 = r#"
            enum Container {
                Item { dynamic val; }
            }
            public fn main(int8 source) -> Container {
                return Container::Item { val: source };
            }
        "#;
        assert!(analyze_src(src3, &cat).is_ok());

        // dynamic structured payload field <- bool -> FieldTypeMismatch
        let src4 = r#"
            enum Container {
                Item { dynamic val; }
            }
            public fn main() -> Container {
                return Container::Item { val: true };
            }
        "#;
        assert!(matches!(
            analyze_src(src4, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::Composite(
                    CompositeFailure::FieldTypeMismatch { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn nested_dynamic_arithmetic_trees() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // (a + b) + c
        let src1 = "public fn main(int8 a, int8 b, int8 c) -> dynamic { return (a + b) + c; }";
        let sem1 = match analyze_src(src1, &cat) {
            Ok(p) => p,
            Err(_) => panic!("(a + b) + c under dynamic should succeed"),
        };
        let dyn_id = sem1
            .types
            .iter()
            .position(|t| matches!(t, SemanticType::Native(NativeType::Dynamic)))
            .unwrap();
        assert_eq!(sem1.functions[0].body.result.type_id.0, dyn_id);
        match &sem1.functions[0].body.result.kind {
            SemanticExpressionKind::Binary { left, right, .. } => {
                assert_eq!(left.type_id.0, dyn_id);
                match &left.kind {
                    SemanticExpressionKind::Binary { .. } => {}
                    _ => panic!("expected nested Binary"),
                }
                assert_ne!(right.type_id.0, dyn_id);
            }
            _ => panic!("expected outer Binary"),
        }

        // a + (b + c)
        let src2 = "public fn main(int8 a, int8 b, int8 c) -> dynamic { return a + (b + c); }";
        let sem2 = match analyze_src(src2, &cat) {
            Ok(p) => p,
            Err(_) => panic!("a + (b + c) under dynamic should succeed"),
        };
        assert_eq!(sem2.functions[0].body.result.type_id.0, dyn_id);
        match &sem2.functions[0].body.result.kind {
            SemanticExpressionKind::Binary { left, right, .. } => {
                assert_ne!(left.type_id.0, dyn_id);
                assert_eq!(right.type_id.0, dyn_id);
                match &right.kind {
                    SemanticExpressionKind::Binary { .. } => {}
                    _ => panic!("expected nested Binary"),
                }
            }
            _ => panic!("expected outer Binary"),
        }
    }

    #[test]
    fn when_numeric_fixed_branches_to_dynamic() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src = r#"
            enum Choice {
                A,
                B
            }

            public fn main(Choice choice, int8 a, int8 b) -> dynamic {
                return when choice {
                    Choice::A => a,
                    Choice::B => b
                };
            }
        "#;
        let sem = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("when branches to dynamic should succeed"),
        };
        let dyn_id = sem
            .types
            .iter()
            .position(|t| matches!(t, SemanticType::Native(NativeType::Dynamic)))
            .unwrap();
        assert_eq!(sem.functions[0].body.result.type_id.0, dyn_id);

        // Non-numeric branch in dynamic when fails
        let src_err = r#"
            enum Choice {
                A,
                B
            }

            public fn main(Choice choice, int8 a) -> dynamic {
                return when choice {
                    Choice::A => a,
                    Choice::B => true
                };
            }
        "#;
        assert!(matches!(
            analyze_src(src_err, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::When(
                    WhenFailure::BranchResultTypeMismatch { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn dynamic_remainder_domain_and_floating_rejection() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // A — Exact normative case: 10.0 % 4.0 -> ArithmeticOperator
        let src_a = "public fn main() -> dynamic { return 10.0 % 4.0; }";
        assert!(matches!(
            analyze_src(src_a, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // B — float32 bindings -> ArithmeticOperator
        let src_b = "public fn main(float32 a, float32 b) -> dynamic { return a % b; }";
        assert!(matches!(
            analyze_src(src_b, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // C — Scientific floating literals -> ArithmeticOperator
        let src_c = "public fn main() -> dynamic { return 1e10 % 2e3; }";
        assert!(matches!(
            analyze_src(src_c, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // D — Unary floating -> ArithmeticOperator
        let src_d = "public fn main() -> dynamic { return -10.0 % 4.0; }";
        assert!(matches!(
            analyze_src(src_d, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // E — Nested known-floating expression -> ArithmeticOperator
        let src_e = "public fn main() -> dynamic { return (1.0 + 2.0) % 3.0; }";
        assert!(matches!(
            analyze_src(src_e, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // F — Fixed integer under dynamic -> success and result.type_id == dynamic
        let src_f = "public fn main(int8 a, int8 b) -> dynamic { return a % b; }";
        let sem_f = match analyze_src(src_f, &cat) {
            Ok(p) => p,
            Err(_) => panic!("fixed integer remainder under dynamic should succeed"),
        };
        let dyn_id = sem_f
            .types
            .iter()
            .position(|t| matches!(t, SemanticType::Native(NativeType::Dynamic)))
            .unwrap();
        assert_eq!(sem_f.functions[0].body.result.type_id.0, dyn_id);

        // G — Integer literals under dynamic -> success
        let src_g = "public fn main() -> dynamic { return 10 % 3; }";
        assert!(analyze_src(src_g, &cat).is_ok());

        // H — True dynamic operands -> success
        let src_h = "public fn main(dynamic a, dynamic b) -> dynamic { return a % b; }";
        assert!(analyze_src(src_h, &cat).is_ok());

        // I — Dynamic plus fixed integer -> success
        let src_i = "public fn main(dynamic a, int8 b) -> dynamic { return a % b; }";
        assert!(analyze_src(src_i, &cat).is_ok());

        // J — Dynamic plus fixed float -> ArithmeticOperator
        let src_j = "public fn main(dynamic a, float32 b) -> dynamic { return a % b; }";
        assert!(matches!(
            analyze_src(src_j, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // (a + b) % c with int8 under dynamic -> success and result.type_id == dynamic
        let src_k = "public fn main(int8 a, int8 b, int8 c) -> dynamic { return (a + b) % c; }";
        let sem_k = match analyze_src(src_k, &cat) {
            Ok(p) => p,
            Err(_) => panic!("(a + b) % c under dynamic should succeed"),
        };
        assert_eq!(sem_k.functions[0].body.result.type_id.0, dyn_id);

        // Nested FieldAccess: (a.inner.value + b.inner.value) % 3 -> ArithmeticOperator
        let src_nested_field = r#"
            struct Inner {
                float value;
            }

            struct Outer {
                Inner inner;
            }

            public fn main(Outer a, Outer b) -> dynamic {
                return (a.inner.value + b.inner.value) % 3;
            }
        "#;
        assert!(matches!(
            analyze_src(src_nested_field, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // Fixed-float-returning Call: (get_value(a) + get_value(b)) % 3 -> ArithmeticOperator
        let src_float_call = r#"
            struct Inner {
                float value;
            }

            struct Outer {
                Inner inner;
            }

            private fn get_value(Outer value) -> float {
                return value.inner.value;
            }

            public fn main(Outer a, Outer b) -> dynamic {
                return (get_value(a) + get_value(b)) % 3;
            }
        "#;
        assert!(matches!(
            analyze_src(src_float_call, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::ArithmeticOperator { .. }
                )),
                ..
            })
        ));

        // Dynamic-returning Call with float argument: unknown(source) % divisor -> semantic success
        let src_dynamic_call = r#"
            private fn unknown(float32 source) -> dynamic {
                return source;
            }

            public fn main(float32 source, dynamic divisor) -> dynamic {
                return unknown(source) % divisor;
            }
        "#;
        assert!(analyze_src(src_dynamic_call, &cat).is_ok());
    }

    #[test]
    fn to_string_domain_restrictions() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        // Numeric -> success
        let src1 = "public fn main(int8 value) -> string { return to_string(value); }";
        assert!(analyze_src(src1, &cat).is_ok());

        // Dynamic -> success
        let src2 = "public fn main(dynamic value) -> string { return to_string(value); }";
        assert!(analyze_src(src2, &cat).is_ok());

        // Bool -> InvalidConversion
        let src3 = "public fn main(bool value) -> string { return to_string(value); }";
        assert!(matches!(
            analyze_src(src3, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::InvalidConversion { .. }
                )),
                ..
            })
        ));

        // String -> InvalidConversion
        let src4 = "public fn main(string value) -> string { return to_string(value); }";
        assert!(matches!(
            analyze_src(src4, &cat),
            Err(CompileFailure {
                kind: CompileFailureKind::Semantic(SemanticFailure::TypeChecking(
                    TypeCheckingFailure::InvalidConversion { .. }
                )),
                ..
            })
        ));
    }

    #[test]
    fn struct_construction_preserves_source_evaluation_order() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src = r#"
            struct Worker {
                int age;
                string name;
            }

            private fn get_name() -> string {
                return "A";
            }

            private fn get_age() -> int {
                return 10;
            }

            public fn main() -> Worker {
                return Worker {
                    name: get_name(),
                    age: get_age()
                };
            }
        "#;

        let sem = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("struct construction should succeed"),
        };

        let main_fn = sem.functions.last().unwrap();
        match &main_fn.body.result.kind {
            SemanticExpressionKind::StructConstruction { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].field.0, 1); // FieldId(name) = 1
                assert_eq!(fields[1].field.0, 0); // FieldId(age) = 0

                match &fields[0].value.kind {
                    SemanticExpressionKind::Call(call) => match &call.target {
                        SemanticCallTarget::Internal(fid) => {
                            assert_eq!(fid.0, 0);
                        }
                        _ => panic!("expected internal call for get_name"),
                    },
                    _ => panic!("expected call for get_name"),
                }

                match &fields[1].value.kind {
                    SemanticExpressionKind::Call(call) => match &call.target {
                        SemanticCallTarget::Internal(fid) => {
                            assert_eq!(fid.0, 1);
                        }
                        _ => panic!("expected internal call for get_age"),
                    },
                    _ => panic!("expected call for get_age"),
                }
            }
            _ => panic!("expected StructConstruction"),
        }
    }

    #[test]
    fn structured_enum_construction_preserves_source_evaluation_order() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };

        let src = r#"
            enum Event {
                Data {
                    int code;
                    string message;
                }
            }

            private fn get_name() -> string {
                return "A";
            }

            private fn get_age() -> int {
                return 10;
            }

            public fn main() -> Event {
                return Event::Data {
                    message: get_name(),
                    code: get_age()
                };
            }
        "#;

        let sem = match analyze_src(src, &cat) {
            Ok(p) => p,
            Err(_) => panic!("structured enum construction should succeed"),
        };

        let main_fn = sem.functions.last().unwrap();
        match &main_fn.body.result.kind {
            SemanticExpressionKind::EnumConstruction { payload, .. } => match payload {
                SemanticEnumPayload::Structured { fields } => {
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].field.0, 1); // FieldId(message) = 1
                    assert_eq!(fields[1].field.0, 0); // FieldId(code) = 0
                }
                _ => panic!("expected structured enum payload"),
            },
            _ => panic!("expected EnumConstruction"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_variant() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { bad_variant } public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::Variant => {}
                _ => panic!("expected Variant role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_function() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn Main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::Function => {}
                _ => panic!("expected Function role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_binding() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "public fn main(int BadParam) -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::Binding => {}
                _ => panic!("expected Binding role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_signature_alias() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "op".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "import svc::op as BadAlias; public fn main() -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::SignatureAlias => {}
                _ => panic!("expected SignatureAlias role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn declaration_invalid_naming_convention_signature_dependency() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "Op".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn main(svc::Op BadDep) -> int { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::Declaration(
                DeclarationFailure::InvalidNamingConvention { role },
            )) => match role {
                SemanticNameRole::SignatureDependency => {}
                _ => panic!("expected SignatureDependency role"),
            },
            _ => panic!("expected InvalidNamingConvention"),
        }
    }

    #[test]
    fn when_variant_not_found() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Ready } public fn main(State s) -> int { return when s { State::Unknown => 1 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(WhenFailure::VariantNotFound {
                variant,
            })) => {
                assert_eq!(&*variant, "Unknown");
            }
            _ => panic!("expected VariantNotFound"),
        }
    }

    #[test]
    fn when_payload_shape_mismatch() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Ready, Payload(int) } public fn main(State s) -> int { return when s { State::Ready(int x) => 1, State::Payload(int x) => 2 }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(
                WhenFailure::PayloadShapeMismatch { expected, actual },
            )) => match (expected, actual) {
                (EnumPayloadShape::Simple, EnumPayloadShape::Associated) => {}
                _ => panic!("expected Simple and Associated"),
            },
            _ => panic!("expected PayloadShapeMismatch"),
        }
    }

    #[test]
    fn when_duplicate_field() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Item { int code; string msg; } } public fn main(State s) -> int { return when s { State::Item { code: int c; code: int c2; } => c }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(WhenFailure::DuplicateField {
                field,
            })) => {
                assert_eq!(&*field, "code");
            }
            _ => panic!("expected DuplicateField"),
        }
    }

    #[test]
    fn when_field_not_found() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Item { int code; } } public fn main(State s) -> int { return when s { State::Item { unknown: int u; } => u }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(WhenFailure::FieldNotFound {
                field,
            })) => {
                assert_eq!(&*field, "unknown");
            }
            _ => panic!("expected FieldNotFound"),
        }
    }

    #[test]
    fn when_missing_field() {
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures: HashMap::new(),
        };
        let src = "enum State { Item { int code; string msg; } } public fn main(State s) -> int { return when s { State::Item { code: int c; } => c }; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::When(WhenFailure::MissingField {
                field,
            })) => {
                assert_eq!(&*field, "msg");
            }
            _ => panic!("expected MissingField"),
        }
    }

    #[test]
    fn signature_mismatch_parameter_kind() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "calc".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::Value(CatalogTypeRef::Int)],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn calc(svc::calc dep) -> int : svc::calc { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                mismatch, ..
            }) => match mismatch {
                SignatureMismatchKind::ParameterKind {
                    position,
                    expected,
                    actual,
                } => {
                    assert_eq!(position, 0);
                    match (expected, actual) {
                        (
                            SemanticArgumentKind::Value,
                            SemanticArgumentKind::SignatureDependency,
                        ) => {}
                        _ => panic!("expected Value and SignatureDependency"),
                    }
                }
                _ => panic!("expected ParameterKind mismatch"),
            },
            _ => panic!("expected SignatureMismatch"),
        }
    }

    #[test]
    fn signature_mismatch_signature_dependency() {
        let mut signatures = HashMap::new();
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "DepA".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Bool,
            },
        );
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "DepB".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![],
                result_type: CatalogTypeRef::Bool,
            },
        );
        signatures.insert(
            SignatureSymbol {
                module: "svc".to_string(),
                name: "runner".to_string(),
            },
            CatalogSignature {
                parameters: alloc::vec![CatalogSignatureParameter::SignatureDependency(
                    SignatureSymbol {
                        module: "svc".to_string(),
                        name: "DepA".to_string(),
                    },
                )],
                result_type: CatalogTypeRef::Int,
            },
        );
        let cat = CompilationCatalog {
            types: HashMap::new(),
            signatures,
        };
        let src = "public fn runner(svc::DepB dep) -> int : svc::runner { return 0; }";
        let err = unwrap_err(analyze_src(src, &cat));
        match err.kind {
            CompileFailureKind::Semantic(SemanticFailure::SignatureMismatch {
                mismatch, ..
            }) => match mismatch {
                SignatureMismatchKind::SignatureDependency {
                    position,
                    expected,
                    actual,
                } => {
                    assert_eq!(position, 0);
                    assert_eq!(expected.name, "DepA");
                    assert_eq!(actual.name, "DepB");
                }
                _ => panic!("expected SignatureDependency mismatch"),
            },
            _ => panic!("expected SignatureMismatch"),
        }
    }
}
