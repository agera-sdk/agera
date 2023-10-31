#![feature(proc_macro_diagnostic)]

// use std::iter::FromIterator;
use proc_macro::TokenStream;
// use proc_macro2::Span;
use quote::{quote, ToTokens};
// use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Pub};
// use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, Token, Path, Visibility, Attribute, Type, Expr, Generics, FnArg, Stmt, braced, WhereClause, parenthesized};

struct EntityInherits {
    /// Types in descending order
    type_sequence: Vec<Path>,
    component_type: Path,
    agera_crate: Option<proc_macro2::TokenStream>,
}

fn parse_full_qualified_id(input: ParseStream) -> Result<Path> {
    Ok(Path::parse_mod_style(input)?)
}

impl Parse for EntityInherits {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut type_sequence: Vec<Path> = vec![];
        type_sequence.push(parse_full_qualified_id(input)?);

        // Super types in descending order
        input.parse::<Token![<]>()?;
        type_sequence.push(parse_full_qualified_id(input)?);
        while input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            type_sequence.push(parse_full_qualified_id(input)?);
        }

        // Component type
        input.parse::<Token![,]>()?;
        input.parse::<Token![use]>()?;
        let component_type = parse_full_qualified_id(input)?;

        let mut agera_crate = None;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            agera_crate = Some(parse_full_qualified_id(input)?.to_token_stream());
        }

        Ok(EntityInherits {
            type_sequence,
            component_type,
            agera_crate,
        })
    }
}

/// Operation used when defining user Entity types. Using `entity_inherits!`,
/// the traits `Deref`, `Clone`, `PartialEq`, `Eq`, `Hash`, `AsRef`, `From`
/// and `TryFrom` are implemented to inherit characteristics from
/// ascending types.
/// 
/// This assumes that the Entity subtype is a struct of the form `struct S(o);`.
/// 
/// # Syntax
/// 
/// The syntax takes a sequence of types in descending order followed by
/// an identifying Component type:
/// 
/// ```ignore
/// entity_inherits!(Subtype < Super1 < SuperN < Entity, use SubtypeComponent);
/// ```
/// 
/// # Notes
/// 
/// This operation is for internal use by Agera. To define Entity subtypes,
/// use simply `entity_type!`.
/// 
#[proc_macro]
pub fn entity_inherits(input: TokenStream) -> TokenStream {
    let EntityInherits { type_sequence, component_type, agera_crate } = parse_macro_input!(input as EntityInherits);
    let subtype = type_sequence[0].clone();
    let super_type = type_sequence[1].clone();
    let agera_crate = agera_crate.unwrap_or(quote! {::agera});

    let mut expanded = TokenStream::new();

    expanded.extend::<TokenStream>(quote! {
        impl ::std::ops::Deref for #subtype { type Target = #super_type; fn deref(&self) -> &Self::Target { &self.0 } }
        impl Clone for #subtype { fn clone(&self) -> Self { Self(self.0.clone()) } }
        impl PartialEq for #subtype { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } }
        impl Eq for #subtype {}
        impl ::std::hash::Hash for #subtype { fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); } }

        // AsRef<SuperType> for Subtype and AsRef<Subtype> for Subtype
        impl AsRef<#super_type> for #subtype { fn as_ref(&self) -> &#super_type { &self.0 } }
        impl AsRef<#subtype> for #subtype { fn as_ref(&self) -> &Self { self } }

        // From<Subtype> for SuperType
        impl From<#subtype> for #super_type { fn from(value: #subtype) -> Self { value.0.clone() } }

        // TryFrom<SuperType> for Subtype
        impl TryFrom<#super_type> for #subtype { type Error = #agera_crate::entity::EntityTypeError; fn try_from(value: #super_type) -> Result<Self, Self::Error> { if value.has::<#component_type>() { Ok(#subtype(value.clone())) } else { Err(#agera_crate::entity::EntityTypeError::new("Type conversion failed")) } } }
    }.into());

    // Indirect super types
    for super_type in &type_sequence[2..] {
        expanded.extend::<TokenStream>(quote! {
            impl AsRef<#super_type> for #subtype { fn as_ref(&self) -> &#super_type { self.0.as_ref() } }
            impl From<#subtype> for #super_type { fn from(value: #subtype) -> Self { #super_type::from(value.0) } }
            impl TryFrom<#super_type> for #subtype { type Error = #agera_crate::entity::EntityTypeError; fn try_from(value: #super_type) -> Result<Self, Self::Error> { if value.has::<#component_type>() { Ok(#subtype(value.try_into()?)) } else { Err(#agera_crate::entity::EntityTypeError::new("Type conversion failed")) } } }
        }.into());
    }

    expanded
}

struct EntityType {
    agera_crate: Option<proc_macro2::TokenStream>,
    attributes: Vec<Attribute>,
    visibility: Visibility,
    name: Ident,
    inherited: Vec<Path>,
    fields: Vec<EntityField>,
    constructor: EntityConstructor,
}

/// A field stores a `RwLock` inside the struct.
struct EntityField {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    is_reference: bool,
    name: Ident,
    type_annotation: Type,
    default_value: Expr,
}

struct EntityConstructor {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    generics: Generics,
    inputs: Punctuated<FnArg, Comma>,
    super_arguments: Punctuated<Expr, Comma>,
    statements: Vec<Stmt>,
}

impl Parse for EntityType {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut agera_crate = None;
        if input.peek(Token![use]) {
            agera_crate = Some(parse_entity_agera_crate_ref(input)?.to_token_stream());
        }

        let attributes = Attribute::parse_outer(input)?;
        let visibility = input.parse::<Visibility>()?;
 
        input.parse::<Token![struct]>()?;
 
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;

        // Inherited
        let mut inherited = vec![];
        inherited.push(Path::parse_mod_style(input)?);
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            inherited.push(Path::parse_mod_style(input)?);
        }

        let mut fields = vec![];
        let braced_content;
        let _ = braced!(braced_content in input);

        while !braced_content.is_empty() {
            fields.push(parse_entity_field(&braced_content)?);
            if braced_content.peek(Token![,]) {
                braced_content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        let mut constructor = EntityConstructor {
            attributes: vec![],
            visibility: Visibility::Public(Pub::default()),
            generics: Generics::default(),
            inputs: Punctuated::new(),
            super_arguments: Punctuated::new(),
            statements: vec![],
        };

        if !input.is_empty() {
            constructor = parse_entity_constructor(input)?;
        }

        Ok(EntityType {
            agera_crate,
            attributes,
            visibility,
            name,
            inherited,
            fields,
            constructor,
        })
    }
}

fn parse_entity_field(input: ParseStream) -> Result<EntityField> {
    let attributes = Attribute::parse_outer(input)?;
    let visibility = input.parse::<Visibility>()?;
    let is_reference = if input.peek(Token![ref]) {
        input.parse::<Token![ref]>()?;
        true
    } else {
        false
    };
    let name = input.parse::<Ident>()?;
    input.parse::<Token![:]>()?;
    let type_annotation = input.parse::<Type>()?;
    input.parse::<Token![=]>()?;
    let default_value = input.parse::<Expr>()?;

    Ok(EntityField {
        attributes,
        visibility,
        is_reference,
        name,
        type_annotation,
        default_value,
    })
}

fn parse_entity_constructor(input: ParseStream) -> Result<EntityConstructor> {
    let attributes = Attribute::parse_outer(input)?;
    let visibility = input.parse::<Visibility>()?;
    input.parse::<Token![fn]>()?;
    let id = input.parse::<Ident>()?;
    if id.to_string() != "constructor" {
        id.span().unwrap().error("Identifier must be equals \"constructor\"").emit();
    }
    let mut generics = input.parse::<Generics>()?;

    let parens_content;
    parenthesized!(parens_content in input);
    let inputs = parens_content.parse_terminated(FnArg::parse, Comma)?;

    generics.where_clause = if input.peek(Token![where]) { Some(input.parse::<WhereClause>()?) } else { None };

    let braced_content;
    let _ = braced!(braced_content in input);
    braced_content.parse::<Token![super]>()?;

    let paren_content;
    let _ = parenthesized!(paren_content in braced_content);
    let super_arguments = paren_content.parse_terminated(Expr::parse, Comma)?;
    braced_content.parse::<Token![;]>()?;

    let mut statements = vec![];
    while !braced_content.is_empty() {
        statements.push(braced_content.parse::<Stmt>()?);
    }

    Ok(EntityConstructor {
        attributes,
        visibility,
        generics,
        inputs,
        super_arguments,
        statements,
    })
}

fn parse_entity_agera_crate_ref(input: ParseStream) -> Result<Path> {
    input.parse::<Token![use]>()?;
    let id = input.parse::<Ident>()?;
    if id.to_string() != "agera" {
        id.span().unwrap().error("Identifier must be equals \"agera\"").emit();
    }
    input.parse::<Token![=]>()?;
    let path = Path::parse_mod_style(input)?;
    input.parse::<Token![;]>()?;
    Ok(path)
}

/// Defines an Entity type. Refer to the Agera documentation on *Inheritance*
/// for full details.
#[proc_macro]
pub fn entity_type(input: TokenStream) -> TokenStream {
    let EntityType {
        agera_crate, attributes, visibility, name, inherited, fields,
        constructor
    } = parse_macro_input!(input as EntityType);

    let super_type = inherited[0].clone();
    let component_name = Ident::new(&(name.to_string() + "Component"), name.span().clone());
    let agera_crate = agera_crate.unwrap_or(quote! {::agera});

    let mut expanded = TokenStream::new();

    let mut constructor_tokens = proc_macro2::TokenStream::new();
    {
        let EntityConstructor {
            attributes,
            visibility,
            generics,
            inputs,
            super_arguments,
            statements,
        } = constructor;

        let mut generics_p = proc_macro2::TokenStream::new();
        let mut generics_w = proc_macro2::TokenStream::new();
        if !generics.params.is_empty() {
            let param_seq = generics.params;
            generics_p.extend::<proc_macro2::TokenStream>(quote! { <#param_seq> }.try_into().unwrap());
        }
        if let Some(w) = generics.where_clause {
            generics_w.extend::<proc_macro2::TokenStream>(quote! { #w }.try_into().unwrap());
        }

        constructor_tokens.extend::<proc_macro2::TokenStream>(quote! {
            #(#attributes)*
            #visibility fn new #generics_p (#inputs) -> Self #generics_w {
                let this = Self(#super_type::new(#super_arguments).set(#component_name::default()).try_into().unwrap());
                #(#statements)*
                this
            }
        }.try_into().unwrap());
    }

    let mut component_fields = proc_macro2::TokenStream::new();
    let mut component_field_defaults = proc_macro2::TokenStream::new();
    let mut field_methods = proc_macro2::TokenStream::new();

    for field in fields {
        let EntityField {
            attributes,
            visibility,
            is_reference,
            name,
            type_annotation,
            default_value,
        } = field;
        let setter_name = Ident::new(&("set_".to_owned() + &name.to_string()), name.span().clone());

        if is_reference {
            component_fields.extend::<proc_macro2::TokenStream>(quote! {
                #name: ::std::sync::RwLock<::std::sync::Arc<#type_annotation>>,
            }.try_into().unwrap());
            component_field_defaults.extend::<proc_macro2::TokenStream>(quote! {
                #name: ::std::sync::RwLock::new(::std::sync::Arc::new(#default_value)),
            }.try_into().unwrap());
            field_methods.extend::<proc_macro2::TokenStream>(quote! {
                #(#attributes)*
                #visibility fn #name(&self) -> ::std::sync::Arc<#type_annotation> {
                    ::std::sync::Arc::clone(&*self.get::<#component_name>().unwrap().#name.read().unwrap())
                }
                #(#attributes)*
                #visibility fn #setter_name(&self, value: ::std::sync::Arc<#type_annotation>) -> Self {
                    *self.get::<#component_name>().unwrap().#name.write().unwrap() = value;
                    self.clone()
                }
            }.try_into().unwrap());
        } else {
            component_fields.extend::<proc_macro2::TokenStream>(quote! {
                #name: ::std::sync::RwLock<#type_annotation>,
            }.try_into().unwrap());
            component_field_defaults.extend::<proc_macro2::TokenStream>(quote! {
                #name: ::std::sync::RwLock::new(#default_value),
            }.try_into().unwrap());
            field_methods.extend::<proc_macro2::TokenStream>(quote! {
                #(#attributes)*
                #visibility fn #name(&self) -> #type_annotation {
                    self.get::<#component_name>().unwrap().#name.read().unwrap().clone()
                }
                #(#attributes)*
                #visibility fn #setter_name(&self, value: #type_annotation) -> Self {
                    *self.get::<#component_name>().unwrap().#name.write().unwrap() = value;
                    self.clone()
                }
            }.try_into().unwrap());
        }
    }

    expanded.extend::<TokenStream>(quote! {
        #(#attributes)*
        #visibility struct #name(#super_type);

        #agera_crate::entity::entity_inherits!(#name < #(#inherited)<*, use #component_name, #agera_crate);

        impl #name {
            #constructor_tokens
            #field_methods
        }

        struct #component_name {
            #component_fields
        }

        impl Default for #component_name {
            fn default() -> Self {
                Self {
                    #component_field_defaults
                }
            }
        }
    }.try_into().unwrap());

    expanded
}