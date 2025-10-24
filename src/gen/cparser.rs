// Generated from C.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::clistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const C_T__0:i32=1; 
		pub const C_T__1:i32=2; 
		pub const C_T__2:i32=3; 
		pub const C_T__3:i32=4; 
		pub const C_T__4:i32=5; 
		pub const C_T__5:i32=6; 
		pub const C_T__6:i32=7; 
		pub const C_T__7:i32=8; 
		pub const C_T__8:i32=9; 
		pub const C_T__9:i32=10; 
		pub const C_T__10:i32=11; 
		pub const C_T__11:i32=12; 
		pub const C_T__12:i32=13; 
		pub const C_T__13:i32=14; 
		pub const C_T__14:i32=15; 
		pub const C_T__15:i32=16; 
		pub const C_T__16:i32=17; 
		pub const C_T__17:i32=18; 
		pub const C_T__18:i32=19; 
		pub const C_Auto:i32=20; 
		pub const C_Break:i32=21; 
		pub const C_Case:i32=22; 
		pub const C_Char:i32=23; 
		pub const C_Const:i32=24; 
		pub const C_Continue:i32=25; 
		pub const C_Default:i32=26; 
		pub const C_Do:i32=27; 
		pub const C_Double:i32=28; 
		pub const C_Else:i32=29; 
		pub const C_Enum:i32=30; 
		pub const C_Extern:i32=31; 
		pub const C_Float:i32=32; 
		pub const C_For:i32=33; 
		pub const C_Goto:i32=34; 
		pub const C_If:i32=35; 
		pub const C_Inline:i32=36; 
		pub const C_Int:i32=37; 
		pub const C_Long:i32=38; 
		pub const C_Register:i32=39; 
		pub const C_Restrict:i32=40; 
		pub const C_Return:i32=41; 
		pub const C_Short:i32=42; 
		pub const C_Signed:i32=43; 
		pub const C_Sizeof:i32=44; 
		pub const C_Static:i32=45; 
		pub const C_Struct:i32=46; 
		pub const C_Switch:i32=47; 
		pub const C_Typedef:i32=48; 
		pub const C_Union:i32=49; 
		pub const C_Unsigned:i32=50; 
		pub const C_Void:i32=51; 
		pub const C_Volatile:i32=52; 
		pub const C_While:i32=53; 
		pub const C_Alignas:i32=54; 
		pub const C_Alignof:i32=55; 
		pub const C_Atomic:i32=56; 
		pub const C_Bool:i32=57; 
		pub const C_Complex:i32=58; 
		pub const C_Generic:i32=59; 
		pub const C_Imaginary:i32=60; 
		pub const C_Noreturn:i32=61; 
		pub const C_StaticAssert:i32=62; 
		pub const C_ThreadLocal:i32=63; 
		pub const C_LeftParen:i32=64; 
		pub const C_RightParen:i32=65; 
		pub const C_LeftBracket:i32=66; 
		pub const C_RightBracket:i32=67; 
		pub const C_LeftBrace:i32=68; 
		pub const C_RightBrace:i32=69; 
		pub const C_Less:i32=70; 
		pub const C_LessEqual:i32=71; 
		pub const C_Greater:i32=72; 
		pub const C_GreaterEqual:i32=73; 
		pub const C_LeftShift:i32=74; 
		pub const C_RightShift:i32=75; 
		pub const C_Plus:i32=76; 
		pub const C_PlusPlus:i32=77; 
		pub const C_Minus:i32=78; 
		pub const C_MinusMinus:i32=79; 
		pub const C_Star:i32=80; 
		pub const C_Div:i32=81; 
		pub const C_Mod:i32=82; 
		pub const C_And:i32=83; 
		pub const C_Or:i32=84; 
		pub const C_AndAnd:i32=85; 
		pub const C_OrOr:i32=86; 
		pub const C_Caret:i32=87; 
		pub const C_Not:i32=88; 
		pub const C_Tilde:i32=89; 
		pub const C_Question:i32=90; 
		pub const C_Colon:i32=91; 
		pub const C_Semi:i32=92; 
		pub const C_Comma:i32=93; 
		pub const C_Assign:i32=94; 
		pub const C_StarAssign:i32=95; 
		pub const C_DivAssign:i32=96; 
		pub const C_ModAssign:i32=97; 
		pub const C_PlusAssign:i32=98; 
		pub const C_MinusAssign:i32=99; 
		pub const C_LeftShiftAssign:i32=100; 
		pub const C_RightShiftAssign:i32=101; 
		pub const C_AndAssign:i32=102; 
		pub const C_XorAssign:i32=103; 
		pub const C_OrAssign:i32=104; 
		pub const C_Equal:i32=105; 
		pub const C_NotEqual:i32=106; 
		pub const C_Arrow:i32=107; 
		pub const C_Dot:i32=108; 
		pub const C_Ellipsis:i32=109; 
		pub const C_Identifier:i32=110; 
		pub const C_Constant:i32=111; 
		pub const C_DigitSequence:i32=112; 
		pub const C_StringLiteral:i32=113; 
		pub const C_MultiLineMacro:i32=114; 
		pub const C_Directive:i32=115; 
		pub const C_AsmBlock:i32=116; 
		pub const C_Whitespace:i32=117; 
		pub const C_Newline:i32=118; 
		pub const C_BlockComment:i32=119; 
		pub const C_LineComment:i32=120;
	pub const C_EOF:i32=EOF;
	pub const RULE_primaryExpression:usize = 0; 
	pub const RULE_genericSelection:usize = 1; 
	pub const RULE_genericAssocList:usize = 2; 
	pub const RULE_genericAssociation:usize = 3; 
	pub const RULE_postfixExpression:usize = 4; 
	pub const RULE_argumentExpressionList:usize = 5; 
	pub const RULE_unaryExpression:usize = 6; 
	pub const RULE_unaryOperator:usize = 7; 
	pub const RULE_castExpression:usize = 8; 
	pub const RULE_multiplicativeExpression:usize = 9; 
	pub const RULE_additiveExpression:usize = 10; 
	pub const RULE_shiftExpression:usize = 11; 
	pub const RULE_relationalExpression:usize = 12; 
	pub const RULE_equalityExpression:usize = 13; 
	pub const RULE_andExpression:usize = 14; 
	pub const RULE_exclusiveOrExpression:usize = 15; 
	pub const RULE_inclusiveOrExpression:usize = 16; 
	pub const RULE_logicalAndExpression:usize = 17; 
	pub const RULE_logicalOrExpression:usize = 18; 
	pub const RULE_conditionalExpression:usize = 19; 
	pub const RULE_assignmentExpression:usize = 20; 
	pub const RULE_assignmentOperator:usize = 21; 
	pub const RULE_expression:usize = 22; 
	pub const RULE_constantExpression:usize = 23; 
	pub const RULE_declaration:usize = 24; 
	pub const RULE_declarationSpecifiers:usize = 25; 
	pub const RULE_declarationSpecifiers2:usize = 26; 
	pub const RULE_declarationSpecifier:usize = 27; 
	pub const RULE_initDeclaratorList:usize = 28; 
	pub const RULE_initDeclarator:usize = 29; 
	pub const RULE_storageClassSpecifier:usize = 30; 
	pub const RULE_typeSpecifier:usize = 31; 
	pub const RULE_structOrUnionSpecifier:usize = 32; 
	pub const RULE_structOrUnion:usize = 33; 
	pub const RULE_structDeclarationList:usize = 34; 
	pub const RULE_structDeclaration:usize = 35; 
	pub const RULE_specifierQualifierList:usize = 36; 
	pub const RULE_structDeclaratorList:usize = 37; 
	pub const RULE_structDeclarator:usize = 38; 
	pub const RULE_enumSpecifier:usize = 39; 
	pub const RULE_enumeratorList:usize = 40; 
	pub const RULE_enumerator:usize = 41; 
	pub const RULE_enumerationConstant:usize = 42; 
	pub const RULE_atomicTypeSpecifier:usize = 43; 
	pub const RULE_typeQualifier:usize = 44; 
	pub const RULE_functionSpecifier:usize = 45; 
	pub const RULE_alignmentSpecifier:usize = 46; 
	pub const RULE_declarator:usize = 47; 
	pub const RULE_directDeclarator:usize = 48; 
	pub const RULE_vcSpecificModifer:usize = 49; 
	pub const RULE_gccDeclaratorExtension:usize = 50; 
	pub const RULE_gccAttributeSpecifier:usize = 51; 
	pub const RULE_gccAttributeList:usize = 52; 
	pub const RULE_gccAttribute:usize = 53; 
	pub const RULE_pointer:usize = 54; 
	pub const RULE_typeQualifierList:usize = 55; 
	pub const RULE_parameterTypeList:usize = 56; 
	pub const RULE_parameterList:usize = 57; 
	pub const RULE_parameterDeclaration:usize = 58; 
	pub const RULE_identifierList:usize = 59; 
	pub const RULE_typeName:usize = 60; 
	pub const RULE_abstractDeclarator:usize = 61; 
	pub const RULE_directAbstractDeclarator:usize = 62; 
	pub const RULE_typedefName:usize = 63; 
	pub const RULE_initializer:usize = 64; 
	pub const RULE_initializerList:usize = 65; 
	pub const RULE_designation:usize = 66; 
	pub const RULE_designatorList:usize = 67; 
	pub const RULE_designator:usize = 68; 
	pub const RULE_staticAssertDeclaration:usize = 69; 
	pub const RULE_statement:usize = 70; 
	pub const RULE_labeledStatement:usize = 71; 
	pub const RULE_compoundStatement:usize = 72; 
	pub const RULE_blockItemList:usize = 73; 
	pub const RULE_blockItem:usize = 74; 
	pub const RULE_expressionStatement:usize = 75; 
	pub const RULE_selectionStatement:usize = 76; 
	pub const RULE_iterationStatement:usize = 77; 
	pub const RULE_forCondition:usize = 78; 
	pub const RULE_forDeclaration:usize = 79; 
	pub const RULE_forExpression:usize = 80; 
	pub const RULE_jumpStatement:usize = 81; 
	pub const RULE_compilationUnit:usize = 82; 
	pub const RULE_translationUnit:usize = 83; 
	pub const RULE_externalDeclaration:usize = 84; 
	pub const RULE_functionDefinition:usize = 85; 
	pub const RULE_declarationList:usize = 86;
	pub const ruleNames: [&'static str; 87] =  [
		"primaryExpression", "genericSelection", "genericAssocList", "genericAssociation", 
		"postfixExpression", "argumentExpressionList", "unaryExpression", "unaryOperator", 
		"castExpression", "multiplicativeExpression", "additiveExpression", "shiftExpression", 
		"relationalExpression", "equalityExpression", "andExpression", "exclusiveOrExpression", 
		"inclusiveOrExpression", "logicalAndExpression", "logicalOrExpression", 
		"conditionalExpression", "assignmentExpression", "assignmentOperator", 
		"expression", "constantExpression", "declaration", "declarationSpecifiers", 
		"declarationSpecifiers2", "declarationSpecifier", "initDeclaratorList", 
		"initDeclarator", "storageClassSpecifier", "typeSpecifier", "structOrUnionSpecifier", 
		"structOrUnion", "structDeclarationList", "structDeclaration", "specifierQualifierList", 
		"structDeclaratorList", "structDeclarator", "enumSpecifier", "enumeratorList", 
		"enumerator", "enumerationConstant", "atomicTypeSpecifier", "typeQualifier", 
		"functionSpecifier", "alignmentSpecifier", "declarator", "directDeclarator", 
		"vcSpecificModifer", "gccDeclaratorExtension", "gccAttributeSpecifier", 
		"gccAttributeList", "gccAttribute", "pointer", "typeQualifierList", "parameterTypeList", 
		"parameterList", "parameterDeclaration", "identifierList", "typeName", 
		"abstractDeclarator", "directAbstractDeclarator", "typedefName", "initializer", 
		"initializerList", "designation", "designatorList", "designator", "staticAssertDeclaration", 
		"statement", "labeledStatement", "compoundStatement", "blockItemList", 
		"blockItem", "expressionStatement", "selectionStatement", "iterationStatement", 
		"forCondition", "forDeclaration", "forExpression", "jumpStatement", "compilationUnit", 
		"translationUnit", "externalDeclaration", "functionDefinition", "declarationList"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;110] = [
		None, Some("'__extension__'"), Some("'__builtin_va_arg'"), Some("'__builtin_offsetof'"), 
		Some("'__m128'"), Some("'__m128d'"), Some("'__m128i'"), Some("'__typeof__'"), 
		Some("'__inline__'"), Some("'__stdcall'"), Some("'__declspec'"), Some("'__cdecl'"), 
		Some("'__clrcall'"), Some("'__fastcall'"), Some("'__thiscall'"), Some("'__vectorcall'"), 
		Some("'__asm'"), Some("'__attribute__'"), Some("'__asm__'"), Some("'__volatile__'"), 
		Some("'auto'"), Some("'break'"), Some("'case'"), Some("'char'"), Some("'const'"), 
		Some("'continue'"), Some("'default'"), Some("'do'"), Some("'double'"), 
		Some("'else'"), Some("'enum'"), Some("'extern'"), Some("'float'"), Some("'for'"), 
		Some("'goto'"), Some("'if'"), Some("'inline'"), Some("'int'"), Some("'long'"), 
		Some("'register'"), Some("'restrict'"), Some("'return'"), Some("'short'"), 
		Some("'signed'"), Some("'sizeof'"), Some("'static'"), Some("'struct'"), 
		Some("'switch'"), Some("'typedef'"), Some("'union'"), Some("'unsigned'"), 
		Some("'void'"), Some("'volatile'"), Some("'while'"), Some("'_Alignas'"), 
		Some("'_Alignof'"), Some("'_Atomic'"), Some("'_Bool'"), Some("'_Complex'"), 
		Some("'_Generic'"), Some("'_Imaginary'"), Some("'_Noreturn'"), Some("'_Static_assert'"), 
		Some("'_Thread_local'"), Some("'('"), Some("')'"), Some("'['"), Some("']'"), 
		Some("'{'"), Some("'}'"), Some("'<'"), Some("'<='"), Some("'>'"), Some("'>='"), 
		Some("'<<'"), Some("'>>'"), Some("'+'"), Some("'++'"), Some("'-'"), Some("'--'"), 
		Some("'*'"), Some("'/'"), Some("'%'"), Some("'&'"), Some("'|'"), Some("'&&'"), 
		Some("'||'"), Some("'^'"), Some("'!'"), Some("'~'"), Some("'?'"), Some("':'"), 
		Some("';'"), Some("','"), Some("'='"), Some("'*='"), Some("'/='"), Some("'%='"), 
		Some("'+='"), Some("'-='"), Some("'<<='"), Some("'>>='"), Some("'&='"), 
		Some("'^='"), Some("'|='"), Some("'=='"), Some("'!='"), Some("'->'"), 
		Some("'.'"), Some("'...'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;121]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, Some("Auto"), Some("Break"), 
		Some("Case"), Some("Char"), Some("Const"), Some("Continue"), Some("Default"), 
		Some("Do"), Some("Double"), Some("Else"), Some("Enum"), Some("Extern"), 
		Some("Float"), Some("For"), Some("Goto"), Some("If"), Some("Inline"), 
		Some("Int"), Some("Long"), Some("Register"), Some("Restrict"), Some("Return"), 
		Some("Short"), Some("Signed"), Some("Sizeof"), Some("Static"), Some("Struct"), 
		Some("Switch"), Some("Typedef"), Some("Union"), Some("Unsigned"), Some("Void"), 
		Some("Volatile"), Some("While"), Some("Alignas"), Some("Alignof"), Some("Atomic"), 
		Some("Bool"), Some("Complex"), Some("Generic"), Some("Imaginary"), Some("Noreturn"), 
		Some("StaticAssert"), Some("ThreadLocal"), Some("LeftParen"), Some("RightParen"), 
		Some("LeftBracket"), Some("RightBracket"), Some("LeftBrace"), Some("RightBrace"), 
		Some("Less"), Some("LessEqual"), Some("Greater"), Some("GreaterEqual"), 
		Some("LeftShift"), Some("RightShift"), Some("Plus"), Some("PlusPlus"), 
		Some("Minus"), Some("MinusMinus"), Some("Star"), Some("Div"), Some("Mod"), 
		Some("And"), Some("Or"), Some("AndAnd"), Some("OrOr"), Some("Caret"), 
		Some("Not"), Some("Tilde"), Some("Question"), Some("Colon"), Some("Semi"), 
		Some("Comma"), Some("Assign"), Some("StarAssign"), Some("DivAssign"), 
		Some("ModAssign"), Some("PlusAssign"), Some("MinusAssign"), Some("LeftShiftAssign"), 
		Some("RightShiftAssign"), Some("AndAssign"), Some("XorAssign"), Some("OrAssign"), 
		Some("Equal"), Some("NotEqual"), Some("Arrow"), Some("Dot"), Some("Ellipsis"), 
		Some("Identifier"), Some("Constant"), Some("DigitSequence"), Some("StringLiteral"), 
		Some("MultiLineMacro"), Some("Directive"), Some("AsmBlock"), Some("Whitespace"), 
		Some("Newline"), Some("BlockComment"), Some("LineComment")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,CParserExt<'input>, I, CParserContextType , dyn CListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type CTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, CParserContextType , dyn CListener<'input> + 'a>;

/// Parser for C grammar
pub struct CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				CParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for CParser
pub trait CParserContext<'input>:
	for<'x> Listenable<dyn CListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=CParserContextType>
{}

antlr4rust::coerce_from!{ 'input : CParserContext<'input> }

impl<'input> CParserContext<'input> for TerminalNode<'input,CParserContextType> {}
impl<'input> CParserContext<'input> for ErrorNode<'input,CParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn CParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn CListener<'input> + 'input }

pub struct CParserContextType;
antlr4rust::tid!{CParserContextType}

impl<'input> ParserNodeType<'input> for CParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn CParserContext<'input> + 'input;
}

impl<'input, I> Deref for CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> CParserExt<'input>{
}
antlr4rust::tid! { CParserExt<'a> }

impl<'input> TokenAware<'input> for CParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for CParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for CParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "C.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn CParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					48 => CParser::<'input,I>::directDeclarator_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					62 => CParser::<'input,I>::directAbstractDeclarator_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn directDeclarator_sempred(_localctx: Option<&DirectDeclaratorContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 9)
				}
				1=>{
					recog.precpred(None, 8)
				}
				2=>{
					recog.precpred(None, 7)
				}
				3=>{
					recog.precpred(None, 6)
				}
				4=>{
					recog.precpred(None, 5)
				}
				5=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
	fn directAbstractDeclarator_sempred(_localctx: Option<&DirectAbstractDeclaratorContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				6=>{
					recog.precpred(None, 5)
				}
				7=>{
					recog.precpred(None, 4)
				}
				8=>{
					recog.precpred(None, 3)
				}
				9=>{
					recog.precpred(None, 2)
				}
				10=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- primaryExpression ----------------
pub type PrimaryExpressionContextAll<'input> = PrimaryExpressionContext<'input>;


pub type PrimaryExpressionContext<'input> = BaseParserRuleContext<'input,PrimaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for PrimaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for PrimaryExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_primaryExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_primaryExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrimaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primaryExpression }
}
antlr4rust::tid!{PrimaryExpressionContextExt<'a>}

impl<'input> PrimaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrimaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<PrimaryExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Constant
/// Returns `None` if there is no child corresponding to token Constant
fn Constant(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Constant, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token StringLiteral in current rule
fn StringLiteral_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token StringLiteral, starting from 0.
/// Returns `None` if number of children corresponding to token StringLiteral is less or equal than `i`.
fn StringLiteral(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_StringLiteral, i)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn genericSelection(&self) -> Option<Rc<GenericSelectionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryExpressionContextAttrs<'input> for PrimaryExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn primaryExpression(&mut self,)
	-> Result<Rc<PrimaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_primaryExpression);
        let mut _localctx: Rc<PrimaryExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(207);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(174);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(175);
					recog.base.match_token(C_Constant,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(177); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(176);
						recog.base.match_token(C_StringLiteral,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(179); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==C_StringLiteral) {break}
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(181);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(182);
					recog.expression()?;

					recog.base.set_state(183);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule genericSelection*/
					recog.base.set_state(185);
					recog.genericSelection()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(187);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_T__0 {
						{
						recog.base.set_state(186);
						recog.base.match_token(C_T__0,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(189);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule compoundStatement*/
					recog.base.set_state(190);
					recog.compoundStatement()?;

					recog.base.set_state(191);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(193);
					recog.base.match_token(C_T__1,&mut recog.err_handler)?;

					recog.base.set_state(194);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(195);
					recog.unaryExpression()?;

					recog.base.set_state(196);
					recog.base.match_token(C_Comma,&mut recog.err_handler)?;

					/*InvokeRule typeName*/
					recog.base.set_state(197);
					recog.typeName()?;

					recog.base.set_state(198);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					recog.base.set_state(200);
					recog.base.match_token(C_T__2,&mut recog.err_handler)?;

					recog.base.set_state(201);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule typeName*/
					recog.base.set_state(202);
					recog.typeName()?;

					recog.base.set_state(203);
					recog.base.match_token(C_Comma,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(204);
					recog.unaryExpression()?;

					recog.base.set_state(205);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- genericSelection ----------------
pub type GenericSelectionContextAll<'input> = GenericSelectionContext<'input>;


pub type GenericSelectionContext<'input> = BaseParserRuleContext<'input,GenericSelectionContextExt<'input>>;

#[derive(Clone)]
pub struct GenericSelectionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GenericSelectionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GenericSelectionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_genericSelection(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_genericSelection(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GenericSelectionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericSelection }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericSelection }
}
antlr4rust::tid!{GenericSelectionContextExt<'a>}

impl<'input> GenericSelectionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GenericSelectionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericSelectionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GenericSelectionContextAttrs<'input>: CParserContext<'input> + BorrowMut<GenericSelectionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Generic
/// Returns `None` if there is no child corresponding to token Generic
fn Generic(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Generic, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}
fn genericAssocList(&self) -> Option<Rc<GenericAssocListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}

}

impl<'input> GenericSelectionContextAttrs<'input> for GenericSelectionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn genericSelection(&mut self,)
	-> Result<Rc<GenericSelectionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericSelectionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_genericSelection);
        let mut _localctx: Rc<GenericSelectionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(209);
			recog.base.match_token(C_Generic,&mut recog.err_handler)?;

			recog.base.set_state(210);
			recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

			/*InvokeRule assignmentExpression*/
			recog.base.set_state(211);
			recog.assignmentExpression()?;

			recog.base.set_state(212);
			recog.base.match_token(C_Comma,&mut recog.err_handler)?;

			/*InvokeRule genericAssocList*/
			recog.base.set_state(213);
			recog.genericAssocList()?;

			recog.base.set_state(214);
			recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- genericAssocList ----------------
pub type GenericAssocListContextAll<'input> = GenericAssocListContext<'input>;


pub type GenericAssocListContext<'input> = BaseParserRuleContext<'input,GenericAssocListContextExt<'input>>;

#[derive(Clone)]
pub struct GenericAssocListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GenericAssocListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GenericAssocListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_genericAssocList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_genericAssocList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GenericAssocListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericAssocList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericAssocList }
}
antlr4rust::tid!{GenericAssocListContextExt<'a>}

impl<'input> GenericAssocListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GenericAssocListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericAssocListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GenericAssocListContextAttrs<'input>: CParserContext<'input> + BorrowMut<GenericAssocListContextExt<'input>>{

fn genericAssociation_all(&self) ->  Vec<Rc<GenericAssociationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn genericAssociation(&self, i: usize) -> Option<Rc<GenericAssociationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> GenericAssocListContextAttrs<'input> for GenericAssocListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn genericAssocList(&mut self,)
	-> Result<Rc<GenericAssocListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericAssocListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_genericAssocList);
        let mut _localctx: Rc<GenericAssocListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule genericAssociation*/
			recog.base.set_state(216);
			recog.genericAssociation()?;

			recog.base.set_state(221);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(217);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				/*InvokeRule genericAssociation*/
				recog.base.set_state(218);
				recog.genericAssociation()?;

				}
				}
				recog.base.set_state(223);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- genericAssociation ----------------
pub type GenericAssociationContextAll<'input> = GenericAssociationContext<'input>;


pub type GenericAssociationContext<'input> = BaseParserRuleContext<'input,GenericAssociationContextExt<'input>>;

#[derive(Clone)]
pub struct GenericAssociationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GenericAssociationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GenericAssociationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_genericAssociation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_genericAssociation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GenericAssociationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_genericAssociation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_genericAssociation }
}
antlr4rust::tid!{GenericAssociationContextExt<'a>}

impl<'input> GenericAssociationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GenericAssociationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GenericAssociationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GenericAssociationContextAttrs<'input>: CParserContext<'input> + BorrowMut<GenericAssociationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Colon, 0)
}
fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Default
/// Returns `None` if there is no child corresponding to token Default
fn Default(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Default, 0)
}

}

impl<'input> GenericAssociationContextAttrs<'input> for GenericAssociationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn genericAssociation(&mut self,)
	-> Result<Rc<GenericAssociationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GenericAssociationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_genericAssociation);
        let mut _localctx: Rc<GenericAssociationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(226);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_T__0 |C_T__3 |C_T__4 |C_T__5 |C_T__6 |C_Char |C_Const |C_Double |C_Enum |
			C_Float |C_Int |C_Long |C_Restrict |C_Short |C_Signed |C_Struct |C_Union |
			C_Unsigned |C_Void |C_Volatile |C_Atomic |C_Bool |C_Complex |C_Identifier 
				=> {
					{
					/*InvokeRule typeName*/
					recog.base.set_state(224);
					recog.typeName()?;

					}
				}

			C_Default 
				=> {
					{
					recog.base.set_state(225);
					recog.base.match_token(C_Default,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(228);
			recog.base.match_token(C_Colon,&mut recog.err_handler)?;

			/*InvokeRule assignmentExpression*/
			recog.base.set_state(229);
			recog.assignmentExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- postfixExpression ----------------
pub type PostfixExpressionContextAll<'input> = PostfixExpressionContext<'input>;


pub type PostfixExpressionContext<'input> = BaseParserRuleContext<'input,PostfixExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PostfixExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for PostfixExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for PostfixExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_postfixExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_postfixExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PostfixExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfixExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfixExpression }
}
antlr4rust::tid!{PostfixExpressionContextExt<'a>}

impl<'input> PostfixExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PostfixExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostfixExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PostfixExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<PostfixExpressionContextExt<'input>>{

fn primaryExpression(&self) -> Option<Rc<PrimaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftParen in current rule
fn LeftParen_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftParen, starting from 0.
/// Returns `None` if number of children corresponding to token LeftParen is less or equal than `i`.
fn LeftParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, i)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token RightParen in current rule
fn RightParen_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightParen, starting from 0.
/// Returns `None` if number of children corresponding to token RightParen is less or equal than `i`.
fn RightParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, i)
}
/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBrace, 0)
}
fn initializerList(&self) -> Option<Rc<InitializerListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBrace, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftBracket in current rule
fn LeftBracket_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftBracket, starting from 0.
/// Returns `None` if number of children corresponding to token LeftBracket is less or equal than `i`.
fn LeftBracket(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBracket, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightBracket in current rule
fn RightBracket_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightBracket, starting from 0.
/// Returns `None` if number of children corresponding to token RightBracket is less or equal than `i`.
fn RightBracket(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBracket, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PlusPlus in current rule
fn PlusPlus_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PlusPlus, starting from 0.
/// Returns `None` if number of children corresponding to token PlusPlus is less or equal than `i`.
fn PlusPlus(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_PlusPlus, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MinusMinus in current rule
fn MinusMinus_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MinusMinus, starting from 0.
/// Returns `None` if number of children corresponding to token MinusMinus is less or equal than `i`.
fn MinusMinus(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_MinusMinus, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Dot in current rule
fn Dot_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Dot, starting from 0.
/// Returns `None` if number of children corresponding to token Dot is less or equal than `i`.
fn Dot(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Dot, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Arrow in current rule
fn Arrow_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Arrow, starting from 0.
/// Returns `None` if number of children corresponding to token Arrow is less or equal than `i`.
fn Arrow(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Arrow, i)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}
fn argumentExpressionList_all(&self) ->  Vec<Rc<ArgumentExpressionListContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn argumentExpressionList(&self, i: usize) -> Option<Rc<ArgumentExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PostfixExpressionContextAttrs<'input> for PostfixExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn postfixExpression(&mut self,)
	-> Result<Rc<PostfixExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostfixExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_postfixExpression);
        let mut _localctx: Rc<PostfixExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(245);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule primaryExpression*/
					recog.base.set_state(231);
					recog.primaryExpression()?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(233);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_T__0 {
						{
						recog.base.set_state(232);
						recog.base.match_token(C_T__0,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(235);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule typeName*/
					recog.base.set_state(236);
					recog.typeName()?;

					recog.base.set_state(237);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					recog.base.set_state(238);
					recog.base.match_token(C_LeftBrace,&mut recog.err_handler)?;

					/*InvokeRule initializerList*/
					recog.base.set_state(239);
					recog.initializerList()?;

					recog.base.set_state(241);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Comma {
						{
						recog.base.set_state(240);
						recog.base.match_token(C_Comma,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(243);
					recog.base.match_token(C_RightBrace,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(262);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 40965) != 0) || _la==C_Arrow || _la==C_Dot {
				{
				recog.base.set_state(260);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				C_LeftBracket 
					=> {
						{
						recog.base.set_state(247);
						recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(248);
						recog.expression()?;

						recog.base.set_state(249);
						recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

						}
					}

				C_LeftParen 
					=> {
						{
						recog.base.set_state(251);
						recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

						recog.base.set_state(253);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
							{
							/*InvokeRule argumentExpressionList*/
							recog.base.set_state(252);
							recog.argumentExpressionList()?;

							}
						}

						recog.base.set_state(255);
						recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

						}
					}

				C_Arrow |C_Dot 
					=> {
						{
						recog.base.set_state(256);
						_la = recog.base.input.la(1);
						if { !(_la==C_Arrow || _la==C_Dot) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						recog.base.set_state(257);
						recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

						}
					}

				C_PlusPlus 
					=> {
						{
						recog.base.set_state(258);
						recog.base.match_token(C_PlusPlus,&mut recog.err_handler)?;

						}
					}

				C_MinusMinus 
					=> {
						{
						recog.base.set_state(259);
						recog.base.match_token(C_MinusMinus,&mut recog.err_handler)?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(264);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- argumentExpressionList ----------------
pub type ArgumentExpressionListContextAll<'input> = ArgumentExpressionListContext<'input>;


pub type ArgumentExpressionListContext<'input> = BaseParserRuleContext<'input,ArgumentExpressionListContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentExpressionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ArgumentExpressionListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ArgumentExpressionListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_argumentExpressionList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_argumentExpressionList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ArgumentExpressionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argumentExpressionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argumentExpressionList }
}
antlr4rust::tid!{ArgumentExpressionListContextExt<'a>}

impl<'input> ArgumentExpressionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ArgumentExpressionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentExpressionListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentExpressionListContextAttrs<'input>: CParserContext<'input> + BorrowMut<ArgumentExpressionListContextExt<'input>>{

fn assignmentExpression_all(&self) ->  Vec<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentExpression(&self, i: usize) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> ArgumentExpressionListContextAttrs<'input> for ArgumentExpressionListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn argumentExpressionList(&mut self,)
	-> Result<Rc<ArgumentExpressionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentExpressionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_argumentExpressionList);
        let mut _localctx: Rc<ArgumentExpressionListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule assignmentExpression*/
			recog.base.set_state(265);
			recog.assignmentExpression()?;

			recog.base.set_state(270);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(266);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				/*InvokeRule assignmentExpression*/
				recog.base.set_state(267);
				recog.assignmentExpression()?;

				}
				}
				recog.base.set_state(272);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- unaryExpression ----------------
pub type UnaryExpressionContextAll<'input> = UnaryExpressionContext<'input>;


pub type UnaryExpressionContext<'input> = BaseParserRuleContext<'input,UnaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for UnaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for UnaryExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_unaryExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_unaryExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for UnaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryExpression }
}
antlr4rust::tid!{UnaryExpressionContextExt<'a>}

impl<'input> UnaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UnaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<UnaryExpressionContextExt<'input>>{

fn postfixExpression(&self) -> Option<Rc<PostfixExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unaryOperator(&self) -> Option<Rc<UnaryOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn castExpression(&self) -> Option<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
/// Retrieves first TerminalNode corresponding to token AndAnd
/// Returns `None` if there is no child corresponding to token AndAnd
fn AndAnd(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_AndAnd, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Sizeof in current rule
fn Sizeof_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Sizeof, starting from 0.
/// Returns `None` if number of children corresponding to token Sizeof is less or equal than `i`.
fn Sizeof(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Sizeof, i)
}
/// Retrieves first TerminalNode corresponding to token Alignof
/// Returns `None` if there is no child corresponding to token Alignof
fn Alignof(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Alignof, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token PlusPlus in current rule
fn PlusPlus_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PlusPlus, starting from 0.
/// Returns `None` if number of children corresponding to token PlusPlus is less or equal than `i`.
fn PlusPlus(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_PlusPlus, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MinusMinus in current rule
fn MinusMinus_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MinusMinus, starting from 0.
/// Returns `None` if number of children corresponding to token MinusMinus is less or equal than `i`.
fn MinusMinus(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_MinusMinus, i)
}

}

impl<'input> UnaryExpressionContextAttrs<'input> for UnaryExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unaryExpression(&mut self,)
	-> Result<Rc<UnaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_unaryExpression);
        let mut _localctx: Rc<UnaryExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(273);
					_la = recog.base.input.la(1);
					if { !(_la==C_Sizeof || _la==C_PlusPlus || _la==C_MinusMinus) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
					} 
				}
				recog.base.set_state(278);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			recog.base.set_state(290);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_T__0 |C_T__1 |C_T__2 |C_Generic |C_LeftParen |C_Identifier |C_Constant |
			C_StringLiteral 
				=> {
					{
					/*InvokeRule postfixExpression*/
					recog.base.set_state(279);
					recog.postfixExpression()?;

					}
				}

			C_Plus |C_Minus |C_Star |C_And |C_Not |C_Tilde 
				=> {
					{
					/*InvokeRule unaryOperator*/
					recog.base.set_state(280);
					recog.unaryOperator()?;

					/*InvokeRule castExpression*/
					recog.base.set_state(281);
					recog.castExpression()?;

					}
				}

			C_Sizeof |C_Alignof 
				=> {
					{
					recog.base.set_state(283);
					_la = recog.base.input.la(1);
					if { !(_la==C_Sizeof || _la==C_Alignof) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(284);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule typeName*/
					recog.base.set_state(285);
					recog.typeName()?;

					recog.base.set_state(286);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

			C_AndAnd 
				=> {
					{
					recog.base.set_state(288);
					recog.base.match_token(C_AndAnd,&mut recog.err_handler)?;

					recog.base.set_state(289);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- unaryOperator ----------------
pub type UnaryOperatorContextAll<'input> = UnaryOperatorContext<'input>;


pub type UnaryOperatorContext<'input> = BaseParserRuleContext<'input,UnaryOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for UnaryOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for UnaryOperatorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_unaryOperator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_unaryOperator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for UnaryOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryOperator }
}
antlr4rust::tid!{UnaryOperatorContextExt<'a>}

impl<'input> UnaryOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UnaryOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryOperatorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryOperatorContextAttrs<'input>: CParserContext<'input> + BorrowMut<UnaryOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_And, 0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Star, 0)
}
/// Retrieves first TerminalNode corresponding to token Plus
/// Returns `None` if there is no child corresponding to token Plus
fn Plus(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Plus, 0)
}
/// Retrieves first TerminalNode corresponding to token Minus
/// Returns `None` if there is no child corresponding to token Minus
fn Minus(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Minus, 0)
}
/// Retrieves first TerminalNode corresponding to token Tilde
/// Returns `None` if there is no child corresponding to token Tilde
fn Tilde(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Tilde, 0)
}
/// Retrieves first TerminalNode corresponding to token Not
/// Returns `None` if there is no child corresponding to token Not
fn Not(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Not, 0)
}

}

impl<'input> UnaryOperatorContextAttrs<'input> for UnaryOperatorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn unaryOperator(&mut self,)
	-> Result<Rc<UnaryOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_unaryOperator);
        let mut _localctx: Rc<UnaryOperatorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(292);
			_la = recog.base.input.la(1);
			if { !(((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12437) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- castExpression ----------------
pub type CastExpressionContextAll<'input> = CastExpressionContext<'input>;


pub type CastExpressionContext<'input> = BaseParserRuleContext<'input,CastExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct CastExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for CastExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for CastExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_castExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_castExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for CastExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_castExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_castExpression }
}
antlr4rust::tid!{CastExpressionContextExt<'a>}

impl<'input> CastExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CastExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CastExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CastExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<CastExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn castExpression(&self) -> Option<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DigitSequence
/// Returns `None` if there is no child corresponding to token DigitSequence
fn DigitSequence(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_DigitSequence, 0)
}

}

impl<'input> CastExpressionContextAttrs<'input> for CastExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn castExpression(&mut self,)
	-> Result<Rc<CastExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CastExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_castExpression);
        let mut _localctx: Rc<CastExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(304);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(295);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_T__0 {
						{
						recog.base.set_state(294);
						recog.base.match_token(C_T__0,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(297);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule typeName*/
					recog.base.set_state(298);
					recog.typeName()?;

					recog.base.set_state(299);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					/*InvokeRule castExpression*/
					recog.base.set_state(300);
					recog.castExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule unaryExpression*/
					recog.base.set_state(302);
					recog.unaryExpression()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(303);
					recog.base.match_token(C_DigitSequence,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- multiplicativeExpression ----------------
pub type MultiplicativeExpressionContextAll<'input> = MultiplicativeExpressionContext<'input>;


pub type MultiplicativeExpressionContext<'input> = BaseParserRuleContext<'input,MultiplicativeExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplicativeExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for MultiplicativeExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_multiplicativeExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_multiplicativeExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MultiplicativeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicativeExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicativeExpression }
}
antlr4rust::tid!{MultiplicativeExpressionContextExt<'a>}

impl<'input> MultiplicativeExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MultiplicativeExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplicativeExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplicativeExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<MultiplicativeExpressionContextExt<'input>>{

fn castExpression_all(&self) ->  Vec<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn castExpression(&self, i: usize) -> Option<Rc<CastExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Star in current rule
fn Star_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Star, starting from 0.
/// Returns `None` if number of children corresponding to token Star is less or equal than `i`.
fn Star(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Star, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Div in current rule
fn Div_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Div, starting from 0.
/// Returns `None` if number of children corresponding to token Div is less or equal than `i`.
fn Div(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Div, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Mod in current rule
fn Mod_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Mod, starting from 0.
/// Returns `None` if number of children corresponding to token Mod is less or equal than `i`.
fn Mod(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Mod, i)
}

}

impl<'input> MultiplicativeExpressionContextAttrs<'input> for MultiplicativeExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn multiplicativeExpression(&mut self,)
	-> Result<Rc<MultiplicativeExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplicativeExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_multiplicativeExpression);
        let mut _localctx: Rc<MultiplicativeExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule castExpression*/
			recog.base.set_state(306);
			recog.castExpression()?;

			recog.base.set_state(311);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & 7) != 0) {
				{
				{
				recog.base.set_state(307);
				_la = recog.base.input.la(1);
				if { !(((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & 7) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule castExpression*/
				recog.base.set_state(308);
				recog.castExpression()?;

				}
				}
				recog.base.set_state(313);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- additiveExpression ----------------
pub type AdditiveExpressionContextAll<'input> = AdditiveExpressionContext<'input>;


pub type AdditiveExpressionContext<'input> = BaseParserRuleContext<'input,AdditiveExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AdditiveExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AdditiveExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_additiveExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_additiveExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AdditiveExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additiveExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additiveExpression }
}
antlr4rust::tid!{AdditiveExpressionContextExt<'a>}

impl<'input> AdditiveExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AdditiveExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AdditiveExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<AdditiveExpressionContextExt<'input>>{

fn multiplicativeExpression_all(&self) ->  Vec<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplicativeExpression(&self, i: usize) -> Option<Rc<MultiplicativeExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Plus in current rule
fn Plus_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Plus, starting from 0.
/// Returns `None` if number of children corresponding to token Plus is less or equal than `i`.
fn Plus(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Plus, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Minus in current rule
fn Minus_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Minus, starting from 0.
/// Returns `None` if number of children corresponding to token Minus is less or equal than `i`.
fn Minus(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Minus, i)
}

}

impl<'input> AdditiveExpressionContextAttrs<'input> for AdditiveExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn additiveExpression(&mut self,)
	-> Result<Rc<AdditiveExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AdditiveExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_additiveExpression);
        let mut _localctx: Rc<AdditiveExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule multiplicativeExpression*/
			recog.base.set_state(314);
			recog.multiplicativeExpression()?;

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Plus || _la==C_Minus {
				{
				{
				recog.base.set_state(315);
				_la = recog.base.input.la(1);
				if { !(_la==C_Plus || _la==C_Minus) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule multiplicativeExpression*/
				recog.base.set_state(316);
				recog.multiplicativeExpression()?;

				}
				}
				recog.base.set_state(321);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- shiftExpression ----------------
pub type ShiftExpressionContextAll<'input> = ShiftExpressionContext<'input>;


pub type ShiftExpressionContext<'input> = BaseParserRuleContext<'input,ShiftExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ShiftExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ShiftExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ShiftExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_shiftExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_shiftExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ShiftExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shiftExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shiftExpression }
}
antlr4rust::tid!{ShiftExpressionContextExt<'a>}

impl<'input> ShiftExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ShiftExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ShiftExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ShiftExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ShiftExpressionContextExt<'input>>{

fn additiveExpression_all(&self) ->  Vec<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn additiveExpression(&self, i: usize) -> Option<Rc<AdditiveExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftShift in current rule
fn LeftShift_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftShift, starting from 0.
/// Returns `None` if number of children corresponding to token LeftShift is less or equal than `i`.
fn LeftShift(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftShift, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightShift in current rule
fn RightShift_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightShift, starting from 0.
/// Returns `None` if number of children corresponding to token RightShift is less or equal than `i`.
fn RightShift(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightShift, i)
}

}

impl<'input> ShiftExpressionContextAttrs<'input> for ShiftExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn shiftExpression(&mut self,)
	-> Result<Rc<ShiftExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ShiftExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_shiftExpression);
        let mut _localctx: Rc<ShiftExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule additiveExpression*/
			recog.base.set_state(322);
			recog.additiveExpression()?;

			recog.base.set_state(327);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_LeftShift || _la==C_RightShift {
				{
				{
				recog.base.set_state(323);
				_la = recog.base.input.la(1);
				if { !(_la==C_LeftShift || _la==C_RightShift) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule additiveExpression*/
				recog.base.set_state(324);
				recog.additiveExpression()?;

				}
				}
				recog.base.set_state(329);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- relationalExpression ----------------
pub type RelationalExpressionContextAll<'input> = RelationalExpressionContext<'input>;


pub type RelationalExpressionContext<'input> = BaseParserRuleContext<'input,RelationalExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct RelationalExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for RelationalExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for RelationalExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_relationalExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_relationalExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for RelationalExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relationalExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relationalExpression }
}
antlr4rust::tid!{RelationalExpressionContextExt<'a>}

impl<'input> RelationalExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RelationalExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationalExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RelationalExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<RelationalExpressionContextExt<'input>>{

fn shiftExpression_all(&self) ->  Vec<Rc<ShiftExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shiftExpression(&self, i: usize) -> Option<Rc<ShiftExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Less in current rule
fn Less_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Less, starting from 0.
/// Returns `None` if number of children corresponding to token Less is less or equal than `i`.
fn Less(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Less, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Greater in current rule
fn Greater_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Greater, starting from 0.
/// Returns `None` if number of children corresponding to token Greater is less or equal than `i`.
fn Greater(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Greater, i)
}
/// Retrieves all `TerminalNode`s corresponding to token LessEqual in current rule
fn LessEqual_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LessEqual, starting from 0.
/// Returns `None` if number of children corresponding to token LessEqual is less or equal than `i`.
fn LessEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LessEqual, i)
}
/// Retrieves all `TerminalNode`s corresponding to token GreaterEqual in current rule
fn GreaterEqual_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token GreaterEqual, starting from 0.
/// Returns `None` if number of children corresponding to token GreaterEqual is less or equal than `i`.
fn GreaterEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_GreaterEqual, i)
}

}

impl<'input> RelationalExpressionContextAttrs<'input> for RelationalExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn relationalExpression(&mut self,)
	-> Result<Rc<RelationalExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationalExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_relationalExpression);
        let mut _localctx: Rc<RelationalExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule shiftExpression*/
			recog.base.set_state(330);
			recog.shiftExpression()?;

			recog.base.set_state(335);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 70)) & !0x3f) == 0 && ((1usize << (_la - 70)) & 15) != 0) {
				{
				{
				recog.base.set_state(331);
				_la = recog.base.input.la(1);
				if { !(((((_la - 70)) & !0x3f) == 0 && ((1usize << (_la - 70)) & 15) != 0)) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule shiftExpression*/
				recog.base.set_state(332);
				recog.shiftExpression()?;

				}
				}
				recog.base.set_state(337);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- equalityExpression ----------------
pub type EqualityExpressionContextAll<'input> = EqualityExpressionContext<'input>;


pub type EqualityExpressionContext<'input> = BaseParserRuleContext<'input,EqualityExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for EqualityExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for EqualityExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_equalityExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_equalityExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EqualityExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equalityExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equalityExpression }
}
antlr4rust::tid!{EqualityExpressionContextExt<'a>}

impl<'input> EqualityExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EqualityExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<EqualityExpressionContextExt<'input>>{

fn relationalExpression_all(&self) ->  Vec<Rc<RelationalExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relationalExpression(&self, i: usize) -> Option<Rc<RelationalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Equal in current rule
fn Equal_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Equal, starting from 0.
/// Returns `None` if number of children corresponding to token Equal is less or equal than `i`.
fn Equal(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Equal, i)
}
/// Retrieves all `TerminalNode`s corresponding to token NotEqual in current rule
fn NotEqual_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NotEqual, starting from 0.
/// Returns `None` if number of children corresponding to token NotEqual is less or equal than `i`.
fn NotEqual(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_NotEqual, i)
}

}

impl<'input> EqualityExpressionContextAttrs<'input> for EqualityExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn equalityExpression(&mut self,)
	-> Result<Rc<EqualityExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_equalityExpression);
        let mut _localctx: Rc<EqualityExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule relationalExpression*/
			recog.base.set_state(338);
			recog.relationalExpression()?;

			recog.base.set_state(343);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Equal || _la==C_NotEqual {
				{
				{
				recog.base.set_state(339);
				_la = recog.base.input.la(1);
				if { !(_la==C_Equal || _la==C_NotEqual) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule relationalExpression*/
				recog.base.set_state(340);
				recog.relationalExpression()?;

				}
				}
				recog.base.set_state(345);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- andExpression ----------------
pub type AndExpressionContextAll<'input> = AndExpressionContext<'input>;


pub type AndExpressionContext<'input> = BaseParserRuleContext<'input,AndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AndExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_andExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_andExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_andExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_andExpression }
}
antlr4rust::tid!{AndExpressionContextExt<'a>}

impl<'input> AndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AndExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AndExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<AndExpressionContextExt<'input>>{

fn equalityExpression_all(&self) ->  Vec<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equalityExpression(&self, i: usize) -> Option<Rc<EqualityExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token And in current rule
fn And_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token And, starting from 0.
/// Returns `None` if number of children corresponding to token And is less or equal than `i`.
fn And(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_And, i)
}

}

impl<'input> AndExpressionContextAttrs<'input> for AndExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn andExpression(&mut self,)
	-> Result<Rc<AndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_andExpression);
        let mut _localctx: Rc<AndExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule equalityExpression*/
			recog.base.set_state(346);
			recog.equalityExpression()?;

			recog.base.set_state(351);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_And {
				{
				{
				recog.base.set_state(347);
				recog.base.match_token(C_And,&mut recog.err_handler)?;

				/*InvokeRule equalityExpression*/
				recog.base.set_state(348);
				recog.equalityExpression()?;

				}
				}
				recog.base.set_state(353);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- exclusiveOrExpression ----------------
pub type ExclusiveOrExpressionContextAll<'input> = ExclusiveOrExpressionContext<'input>;


pub type ExclusiveOrExpressionContext<'input> = BaseParserRuleContext<'input,ExclusiveOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExclusiveOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ExclusiveOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ExclusiveOrExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_exclusiveOrExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_exclusiveOrExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ExclusiveOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exclusiveOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exclusiveOrExpression }
}
antlr4rust::tid!{ExclusiveOrExpressionContextExt<'a>}

impl<'input> ExclusiveOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExclusiveOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExclusiveOrExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExclusiveOrExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ExclusiveOrExpressionContextExt<'input>>{

fn andExpression_all(&self) ->  Vec<Rc<AndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn andExpression(&self, i: usize) -> Option<Rc<AndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Caret in current rule
fn Caret_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Caret, starting from 0.
/// Returns `None` if number of children corresponding to token Caret is less or equal than `i`.
fn Caret(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Caret, i)
}

}

impl<'input> ExclusiveOrExpressionContextAttrs<'input> for ExclusiveOrExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn exclusiveOrExpression(&mut self,)
	-> Result<Rc<ExclusiveOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExclusiveOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_exclusiveOrExpression);
        let mut _localctx: Rc<ExclusiveOrExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule andExpression*/
			recog.base.set_state(354);
			recog.andExpression()?;

			recog.base.set_state(359);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Caret {
				{
				{
				recog.base.set_state(355);
				recog.base.match_token(C_Caret,&mut recog.err_handler)?;

				/*InvokeRule andExpression*/
				recog.base.set_state(356);
				recog.andExpression()?;

				}
				}
				recog.base.set_state(361);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- inclusiveOrExpression ----------------
pub type InclusiveOrExpressionContextAll<'input> = InclusiveOrExpressionContext<'input>;


pub type InclusiveOrExpressionContext<'input> = BaseParserRuleContext<'input,InclusiveOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct InclusiveOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for InclusiveOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for InclusiveOrExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_inclusiveOrExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_inclusiveOrExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for InclusiveOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inclusiveOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inclusiveOrExpression }
}
antlr4rust::tid!{InclusiveOrExpressionContextExt<'a>}

impl<'input> InclusiveOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InclusiveOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InclusiveOrExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InclusiveOrExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<InclusiveOrExpressionContextExt<'input>>{

fn exclusiveOrExpression_all(&self) ->  Vec<Rc<ExclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn exclusiveOrExpression(&self, i: usize) -> Option<Rc<ExclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Or in current rule
fn Or_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Or, starting from 0.
/// Returns `None` if number of children corresponding to token Or is less or equal than `i`.
fn Or(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Or, i)
}

}

impl<'input> InclusiveOrExpressionContextAttrs<'input> for InclusiveOrExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn inclusiveOrExpression(&mut self,)
	-> Result<Rc<InclusiveOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InclusiveOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_inclusiveOrExpression);
        let mut _localctx: Rc<InclusiveOrExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule exclusiveOrExpression*/
			recog.base.set_state(362);
			recog.exclusiveOrExpression()?;

			recog.base.set_state(367);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Or {
				{
				{
				recog.base.set_state(363);
				recog.base.match_token(C_Or,&mut recog.err_handler)?;

				/*InvokeRule exclusiveOrExpression*/
				recog.base.set_state(364);
				recog.exclusiveOrExpression()?;

				}
				}
				recog.base.set_state(369);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- logicalAndExpression ----------------
pub type LogicalAndExpressionContextAll<'input> = LogicalAndExpressionContext<'input>;


pub type LogicalAndExpressionContext<'input> = BaseParserRuleContext<'input,LogicalAndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalAndExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for LogicalAndExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for LogicalAndExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_logicalAndExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_logicalAndExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LogicalAndExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalAndExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalAndExpression }
}
antlr4rust::tid!{LogicalAndExpressionContextExt<'a>}

impl<'input> LogicalAndExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LogicalAndExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalAndExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LogicalAndExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<LogicalAndExpressionContextExt<'input>>{

fn inclusiveOrExpression_all(&self) ->  Vec<Rc<InclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inclusiveOrExpression(&self, i: usize) -> Option<Rc<InclusiveOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AndAnd in current rule
fn AndAnd_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AndAnd, starting from 0.
/// Returns `None` if number of children corresponding to token AndAnd is less or equal than `i`.
fn AndAnd(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_AndAnd, i)
}

}

impl<'input> LogicalAndExpressionContextAttrs<'input> for LogicalAndExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn logicalAndExpression(&mut self,)
	-> Result<Rc<LogicalAndExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalAndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_logicalAndExpression);
        let mut _localctx: Rc<LogicalAndExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule inclusiveOrExpression*/
			recog.base.set_state(370);
			recog.inclusiveOrExpression()?;

			recog.base.set_state(375);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_AndAnd {
				{
				{
				recog.base.set_state(371);
				recog.base.match_token(C_AndAnd,&mut recog.err_handler)?;

				/*InvokeRule inclusiveOrExpression*/
				recog.base.set_state(372);
				recog.inclusiveOrExpression()?;

				}
				}
				recog.base.set_state(377);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- logicalOrExpression ----------------
pub type LogicalOrExpressionContextAll<'input> = LogicalOrExpressionContext<'input>;


pub type LogicalOrExpressionContext<'input> = BaseParserRuleContext<'input,LogicalOrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct LogicalOrExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for LogicalOrExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for LogicalOrExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_logicalOrExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_logicalOrExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LogicalOrExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicalOrExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicalOrExpression }
}
antlr4rust::tid!{LogicalOrExpressionContextExt<'a>}

impl<'input> LogicalOrExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LogicalOrExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicalOrExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LogicalOrExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<LogicalOrExpressionContextExt<'input>>{

fn logicalAndExpression_all(&self) ->  Vec<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logicalAndExpression(&self, i: usize) -> Option<Rc<LogicalAndExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token OrOr in current rule
fn OrOr_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OrOr, starting from 0.
/// Returns `None` if number of children corresponding to token OrOr is less or equal than `i`.
fn OrOr(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_OrOr, i)
}

}

impl<'input> LogicalOrExpressionContextAttrs<'input> for LogicalOrExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn logicalOrExpression(&mut self,)
	-> Result<Rc<LogicalOrExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicalOrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_logicalOrExpression);
        let mut _localctx: Rc<LogicalOrExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule logicalAndExpression*/
			recog.base.set_state(378);
			recog.logicalAndExpression()?;

			recog.base.set_state(383);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_OrOr {
				{
				{
				recog.base.set_state(379);
				recog.base.match_token(C_OrOr,&mut recog.err_handler)?;

				/*InvokeRule logicalAndExpression*/
				recog.base.set_state(380);
				recog.logicalAndExpression()?;

				}
				}
				recog.base.set_state(385);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- conditionalExpression ----------------
pub type ConditionalExpressionContextAll<'input> = ConditionalExpressionContext<'input>;


pub type ConditionalExpressionContext<'input> = BaseParserRuleContext<'input,ConditionalExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConditionalExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ConditionalExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ConditionalExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_conditionalExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_conditionalExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ConditionalExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_conditionalExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_conditionalExpression }
}
antlr4rust::tid!{ConditionalExpressionContextExt<'a>}

impl<'input> ConditionalExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConditionalExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConditionalExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConditionalExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ConditionalExpressionContextExt<'input>>{

fn logicalOrExpression(&self) -> Option<Rc<LogicalOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Question
/// Returns `None` if there is no child corresponding to token Question
fn Question(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Question, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Colon, 0)
}
fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConditionalExpressionContextAttrs<'input> for ConditionalExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn conditionalExpression(&mut self,)
	-> Result<Rc<ConditionalExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConditionalExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_conditionalExpression);
        let mut _localctx: Rc<ConditionalExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule logicalOrExpression*/
			recog.base.set_state(386);
			recog.logicalOrExpression()?;

			recog.base.set_state(392);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_Question {
				{
				recog.base.set_state(387);
				recog.base.match_token(C_Question,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(388);
				recog.expression()?;

				recog.base.set_state(389);
				recog.base.match_token(C_Colon,&mut recog.err_handler)?;

				/*InvokeRule conditionalExpression*/
				recog.base.set_state(390);
				recog.conditionalExpression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- assignmentExpression ----------------
pub type AssignmentExpressionContextAll<'input> = AssignmentExpressionContext<'input>;


pub type AssignmentExpressionContext<'input> = BaseParserRuleContext<'input,AssignmentExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AssignmentExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AssignmentExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_assignmentExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_assignmentExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AssignmentExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentExpression }
}
antlr4rust::tid!{AssignmentExpressionContextExt<'a>}

impl<'input> AssignmentExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AssignmentExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<AssignmentExpressionContextExt<'input>>{

fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentOperator(&self) -> Option<Rc<AssignmentOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DigitSequence
/// Returns `None` if there is no child corresponding to token DigitSequence
fn DigitSequence(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_DigitSequence, 0)
}

}

impl<'input> AssignmentExpressionContextAttrs<'input> for AssignmentExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn assignmentExpression(&mut self,)
	-> Result<Rc<AssignmentExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_assignmentExpression);
        let mut _localctx: Rc<AssignmentExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(400);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule conditionalExpression*/
					recog.base.set_state(394);
					recog.conditionalExpression()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule unaryExpression*/
					recog.base.set_state(395);
					recog.unaryExpression()?;

					/*InvokeRule assignmentOperator*/
					recog.base.set_state(396);
					recog.assignmentOperator()?;

					/*InvokeRule assignmentExpression*/
					recog.base.set_state(397);
					recog.assignmentExpression()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(399);
					recog.base.match_token(C_DigitSequence,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- assignmentOperator ----------------
pub type AssignmentOperatorContextAll<'input> = AssignmentOperatorContext<'input>;


pub type AssignmentOperatorContext<'input> = BaseParserRuleContext<'input,AssignmentOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct AssignmentOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AssignmentOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AssignmentOperatorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_assignmentOperator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_assignmentOperator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AssignmentOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignmentOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignmentOperator }
}
antlr4rust::tid!{AssignmentOperatorContextExt<'a>}

impl<'input> AssignmentOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AssignmentOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignmentOperatorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AssignmentOperatorContextAttrs<'input>: CParserContext<'input> + BorrowMut<AssignmentOperatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Assign
/// Returns `None` if there is no child corresponding to token Assign
fn Assign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Assign, 0)
}
/// Retrieves first TerminalNode corresponding to token StarAssign
/// Returns `None` if there is no child corresponding to token StarAssign
fn StarAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_StarAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token DivAssign
/// Returns `None` if there is no child corresponding to token DivAssign
fn DivAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_DivAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token ModAssign
/// Returns `None` if there is no child corresponding to token ModAssign
fn ModAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_ModAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token PlusAssign
/// Returns `None` if there is no child corresponding to token PlusAssign
fn PlusAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_PlusAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token MinusAssign
/// Returns `None` if there is no child corresponding to token MinusAssign
fn MinusAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_MinusAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftShiftAssign
/// Returns `None` if there is no child corresponding to token LeftShiftAssign
fn LeftShiftAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftShiftAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token RightShiftAssign
/// Returns `None` if there is no child corresponding to token RightShiftAssign
fn RightShiftAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightShiftAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token AndAssign
/// Returns `None` if there is no child corresponding to token AndAssign
fn AndAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_AndAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token XorAssign
/// Returns `None` if there is no child corresponding to token XorAssign
fn XorAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_XorAssign, 0)
}
/// Retrieves first TerminalNode corresponding to token OrAssign
/// Returns `None` if there is no child corresponding to token OrAssign
fn OrAssign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_OrAssign, 0)
}

}

impl<'input> AssignmentOperatorContextAttrs<'input> for AssignmentOperatorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn assignmentOperator(&mut self,)
	-> Result<Rc<AssignmentOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignmentOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_assignmentOperator);
        let mut _localctx: Rc<AssignmentOperatorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(402);
			_la = recog.base.input.la(1);
			if { !(((((_la - 94)) & !0x3f) == 0 && ((1usize << (_la - 94)) & 2047) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_expression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_expression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr4rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn assignmentExpression_all(&self) ->  Vec<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentExpression(&self, i: usize) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule assignmentExpression*/
			recog.base.set_state(404);
			recog.assignmentExpression()?;

			recog.base.set_state(409);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(405);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				/*InvokeRule assignmentExpression*/
				recog.base.set_state(406);
				recog.assignmentExpression()?;

				}
				}
				recog.base.set_state(411);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- constantExpression ----------------
pub type ConstantExpressionContextAll<'input> = ConstantExpressionContext<'input>;


pub type ConstantExpressionContext<'input> = BaseParserRuleContext<'input,ConstantExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ConstantExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ConstantExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_constantExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_constantExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ConstantExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constantExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constantExpression }
}
antlr4rust::tid!{ConstantExpressionContextExt<'a>}

impl<'input> ConstantExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConstantExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ConstantExpressionContextExt<'input>>{

fn conditionalExpression(&self) -> Option<Rc<ConditionalExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ConstantExpressionContextAttrs<'input> for ConstantExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn constantExpression(&mut self,)
	-> Result<Rc<ConstantExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_constantExpression);
        let mut _localctx: Rc<ConstantExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule conditionalExpression*/
			recog.base.set_state(412);
			recog.conditionalExpression()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declaration ----------------
pub type DeclarationContextAll<'input> = DeclarationContext<'input>;


pub type DeclarationContext<'input> = BaseParserRuleContext<'input,DeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declaration(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declaration }
}
antlr4rust::tid!{DeclarationContextExt<'a>}

impl<'input> DeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationContextAttrs<'input>: CParserContext<'input> + BorrowMut<DeclarationContextExt<'input>>{

fn declarationSpecifiers(&self) -> Option<Rc<DeclarationSpecifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
fn initDeclaratorList(&self) -> Option<Rc<InitDeclaratorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn staticAssertDeclaration(&self) -> Option<Rc<StaticAssertDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declaration(&mut self,)
	-> Result<Rc<DeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_declaration);
        let mut _localctx: Rc<DeclarationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(421);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_T__0 |C_T__3 |C_T__4 |C_T__5 |C_T__6 |C_T__7 |C_T__8 |C_T__9 |C_T__16 |
			C_Auto |C_Char |C_Const |C_Double |C_Enum |C_Extern |C_Float |C_Inline |
			C_Int |C_Long |C_Register |C_Restrict |C_Short |C_Signed |C_Static |C_Struct |
			C_Typedef |C_Union |C_Unsigned |C_Void |C_Volatile |C_Alignas |C_Atomic |
			C_Bool |C_Complex |C_Noreturn |C_ThreadLocal |C_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule declarationSpecifiers*/
					recog.base.set_state(414);
					recog.declarationSpecifiers()?;

					recog.base.set_state(416);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 64000) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 8454145) != 0) || _la==C_Identifier {
						{
						/*InvokeRule initDeclaratorList*/
						recog.base.set_state(415);
						recog.initDeclaratorList()?;

						}
					}

					recog.base.set_state(418);
					recog.base.match_token(C_Semi,&mut recog.err_handler)?;

					}
				}

			C_StaticAssert 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule staticAssertDeclaration*/
					recog.base.set_state(420);
					recog.staticAssertDeclaration()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declarationSpecifiers ----------------
pub type DeclarationSpecifiersContextAll<'input> = DeclarationSpecifiersContext<'input>;


pub type DeclarationSpecifiersContext<'input> = BaseParserRuleContext<'input,DeclarationSpecifiersContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationSpecifiersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DeclarationSpecifiersContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DeclarationSpecifiersContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declarationSpecifiers(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declarationSpecifiers(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationSpecifiersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationSpecifiers }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationSpecifiers }
}
antlr4rust::tid!{DeclarationSpecifiersContextExt<'a>}

impl<'input> DeclarationSpecifiersContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclarationSpecifiersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationSpecifiersContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationSpecifiersContextAttrs<'input>: CParserContext<'input> + BorrowMut<DeclarationSpecifiersContextExt<'input>>{

fn declarationSpecifier_all(&self) ->  Vec<Rc<DeclarationSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declarationSpecifier(&self, i: usize) -> Option<Rc<DeclarationSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DeclarationSpecifiersContextAttrs<'input> for DeclarationSpecifiersContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declarationSpecifiers(&mut self,)
	-> Result<Rc<DeclarationSpecifiersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationSpecifiersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_declarationSpecifiers);
        let mut _localctx: Rc<DeclarationSpecifiersContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(424); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule declarationSpecifier*/
					recog.base.set_state(423);
					recog.declarationSpecifier()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(426); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(31,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declarationSpecifiers2 ----------------
pub type DeclarationSpecifiers2ContextAll<'input> = DeclarationSpecifiers2Context<'input>;


pub type DeclarationSpecifiers2Context<'input> = BaseParserRuleContext<'input,DeclarationSpecifiers2ContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationSpecifiers2ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DeclarationSpecifiers2Context<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DeclarationSpecifiers2Context<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declarationSpecifiers2(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declarationSpecifiers2(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationSpecifiers2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationSpecifiers2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationSpecifiers2 }
}
antlr4rust::tid!{DeclarationSpecifiers2ContextExt<'a>}

impl<'input> DeclarationSpecifiers2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclarationSpecifiers2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationSpecifiers2ContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationSpecifiers2ContextAttrs<'input>: CParserContext<'input> + BorrowMut<DeclarationSpecifiers2ContextExt<'input>>{

fn declarationSpecifier_all(&self) ->  Vec<Rc<DeclarationSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declarationSpecifier(&self, i: usize) -> Option<Rc<DeclarationSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DeclarationSpecifiers2ContextAttrs<'input> for DeclarationSpecifiers2Context<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declarationSpecifiers2(&mut self,)
	-> Result<Rc<DeclarationSpecifiers2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationSpecifiers2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_declarationSpecifiers2);
        let mut _localctx: Rc<DeclarationSpecifiers2ContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(429); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule declarationSpecifier*/
				recog.base.set_state(428);
				recog.declarationSpecifier()?;

				}
				}
				recog.base.set_state(431); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 3516008434) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2808049137) != 0) || _la==C_Identifier) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declarationSpecifier ----------------
pub type DeclarationSpecifierContextAll<'input> = DeclarationSpecifierContext<'input>;


pub type DeclarationSpecifierContext<'input> = BaseParserRuleContext<'input,DeclarationSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DeclarationSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DeclarationSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declarationSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declarationSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationSpecifier }
}
antlr4rust::tid!{DeclarationSpecifierContextExt<'a>}

impl<'input> DeclarationSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclarationSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<DeclarationSpecifierContextExt<'input>>{

fn storageClassSpecifier(&self) -> Option<Rc<StorageClassSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeSpecifier(&self) -> Option<Rc<TypeSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeQualifier(&self) -> Option<Rc<TypeQualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionSpecifier(&self) -> Option<Rc<FunctionSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn alignmentSpecifier(&self) -> Option<Rc<AlignmentSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationSpecifierContextAttrs<'input> for DeclarationSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declarationSpecifier(&mut self,)
	-> Result<Rc<DeclarationSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_declarationSpecifier);
        let mut _localctx: Rc<DeclarationSpecifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(438);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule storageClassSpecifier*/
					recog.base.set_state(433);
					recog.storageClassSpecifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule typeSpecifier*/
					recog.base.set_state(434);
					recog.typeSpecifier()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule typeQualifier*/
					recog.base.set_state(435);
					recog.typeQualifier()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule functionSpecifier*/
					recog.base.set_state(436);
					recog.functionSpecifier()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule alignmentSpecifier*/
					recog.base.set_state(437);
					recog.alignmentSpecifier()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- initDeclaratorList ----------------
pub type InitDeclaratorListContextAll<'input> = InitDeclaratorListContext<'input>;


pub type InitDeclaratorListContext<'input> = BaseParserRuleContext<'input,InitDeclaratorListContextExt<'input>>;

#[derive(Clone)]
pub struct InitDeclaratorListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for InitDeclaratorListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for InitDeclaratorListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_initDeclaratorList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_initDeclaratorList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for InitDeclaratorListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initDeclaratorList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initDeclaratorList }
}
antlr4rust::tid!{InitDeclaratorListContextExt<'a>}

impl<'input> InitDeclaratorListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InitDeclaratorListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitDeclaratorListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InitDeclaratorListContextAttrs<'input>: CParserContext<'input> + BorrowMut<InitDeclaratorListContextExt<'input>>{

fn initDeclarator_all(&self) ->  Vec<Rc<InitDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn initDeclarator(&self, i: usize) -> Option<Rc<InitDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> InitDeclaratorListContextAttrs<'input> for InitDeclaratorListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn initDeclaratorList(&mut self,)
	-> Result<Rc<InitDeclaratorListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitDeclaratorListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_initDeclaratorList);
        let mut _localctx: Rc<InitDeclaratorListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule initDeclarator*/
			recog.base.set_state(440);
			recog.initDeclarator()?;

			recog.base.set_state(445);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(441);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				/*InvokeRule initDeclarator*/
				recog.base.set_state(442);
				recog.initDeclarator()?;

				}
				}
				recog.base.set_state(447);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- initDeclarator ----------------
pub type InitDeclaratorContextAll<'input> = InitDeclaratorContext<'input>;


pub type InitDeclaratorContext<'input> = BaseParserRuleContext<'input,InitDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct InitDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for InitDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for InitDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_initDeclarator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_initDeclarator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for InitDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initDeclarator }
}
antlr4rust::tid!{InitDeclaratorContextExt<'a>}

impl<'input> InitDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InitDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitDeclaratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InitDeclaratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<InitDeclaratorContextExt<'input>>{

fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Assign
/// Returns `None` if there is no child corresponding to token Assign
fn Assign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Assign, 0)
}
fn initializer(&self) -> Option<Rc<InitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InitDeclaratorContextAttrs<'input> for InitDeclaratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn initDeclarator(&mut self,)
	-> Result<Rc<InitDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_initDeclarator);
        let mut _localctx: Rc<InitDeclaratorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule declarator*/
			recog.base.set_state(448);
			recog.declarator()?;

			recog.base.set_state(451);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_Assign {
				{
				recog.base.set_state(449);
				recog.base.match_token(C_Assign,&mut recog.err_handler)?;

				/*InvokeRule initializer*/
				recog.base.set_state(450);
				recog.initializer()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- storageClassSpecifier ----------------
pub type StorageClassSpecifierContextAll<'input> = StorageClassSpecifierContext<'input>;


pub type StorageClassSpecifierContext<'input> = BaseParserRuleContext<'input,StorageClassSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct StorageClassSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StorageClassSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StorageClassSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_storageClassSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_storageClassSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StorageClassSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_storageClassSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_storageClassSpecifier }
}
antlr4rust::tid!{StorageClassSpecifierContextExt<'a>}

impl<'input> StorageClassSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StorageClassSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StorageClassSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StorageClassSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<StorageClassSpecifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Typedef
/// Returns `None` if there is no child corresponding to token Typedef
fn Typedef(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Typedef, 0)
}
/// Retrieves first TerminalNode corresponding to token Extern
/// Returns `None` if there is no child corresponding to token Extern
fn Extern(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Extern, 0)
}
/// Retrieves first TerminalNode corresponding to token Static
/// Returns `None` if there is no child corresponding to token Static
fn Static(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Static, 0)
}
/// Retrieves first TerminalNode corresponding to token ThreadLocal
/// Returns `None` if there is no child corresponding to token ThreadLocal
fn ThreadLocal(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_ThreadLocal, 0)
}
/// Retrieves first TerminalNode corresponding to token Auto
/// Returns `None` if there is no child corresponding to token Auto
fn Auto(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Auto, 0)
}
/// Retrieves first TerminalNode corresponding to token Register
/// Returns `None` if there is no child corresponding to token Register
fn Register(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Register, 0)
}

}

impl<'input> StorageClassSpecifierContextAttrs<'input> for StorageClassSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn storageClassSpecifier(&mut self,)
	-> Result<Rc<StorageClassSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StorageClassSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_storageClassSpecifier);
        let mut _localctx: Rc<StorageClassSpecifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(453);
			_la = recog.base.input.la(1);
			if { !(_la==C_Auto || _la==C_Extern || ((((_la - 39)) & !0x3f) == 0 && ((1usize << (_la - 39)) & 16777793) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- typeSpecifier ----------------
pub type TypeSpecifierContextAll<'input> = TypeSpecifierContext<'input>;


pub type TypeSpecifierContext<'input> = BaseParserRuleContext<'input,TypeSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct TypeSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for TypeSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for TypeSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TypeSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeSpecifier }
}
antlr4rust::tid!{TypeSpecifierContextExt<'a>}

impl<'input> TypeSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<TypeSpecifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Void
/// Returns `None` if there is no child corresponding to token Void
fn Void(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Void, 0)
}
/// Retrieves first TerminalNode corresponding to token Char
/// Returns `None` if there is no child corresponding to token Char
fn Char(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Char, 0)
}
/// Retrieves first TerminalNode corresponding to token Short
/// Returns `None` if there is no child corresponding to token Short
fn Short(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Short, 0)
}
/// Retrieves first TerminalNode corresponding to token Int
/// Returns `None` if there is no child corresponding to token Int
fn Int(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Int, 0)
}
/// Retrieves first TerminalNode corresponding to token Long
/// Returns `None` if there is no child corresponding to token Long
fn Long(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Long, 0)
}
/// Retrieves first TerminalNode corresponding to token Float
/// Returns `None` if there is no child corresponding to token Float
fn Float(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Float, 0)
}
/// Retrieves first TerminalNode corresponding to token Double
/// Returns `None` if there is no child corresponding to token Double
fn Double(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Double, 0)
}
/// Retrieves first TerminalNode corresponding to token Signed
/// Returns `None` if there is no child corresponding to token Signed
fn Signed(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Signed, 0)
}
/// Retrieves first TerminalNode corresponding to token Unsigned
/// Returns `None` if there is no child corresponding to token Unsigned
fn Unsigned(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Unsigned, 0)
}
/// Retrieves first TerminalNode corresponding to token Bool
/// Returns `None` if there is no child corresponding to token Bool
fn Bool(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Bool, 0)
}
/// Retrieves first TerminalNode corresponding to token Complex
/// Returns `None` if there is no child corresponding to token Complex
fn Complex(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Complex, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn atomicTypeSpecifier(&self) -> Option<Rc<AtomicTypeSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structOrUnionSpecifier(&self) -> Option<Rc<StructOrUnionSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumSpecifier(&self) -> Option<Rc<EnumSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typedefName(&self) -> Option<Rc<TypedefNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeSpecifierContextAttrs<'input> for TypeSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeSpecifier(&mut self,)
	-> Result<Rc<TypeSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_typeSpecifier);
        let mut _localctx: Rc<TypeSpecifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_Void 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(455);
					recog.base.match_token(C_Void,&mut recog.err_handler)?;

					}
				}

			C_Char 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(456);
					recog.base.match_token(C_Char,&mut recog.err_handler)?;

					}
				}

			C_Short 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(457);
					recog.base.match_token(C_Short,&mut recog.err_handler)?;

					}
				}

			C_Int 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(458);
					recog.base.match_token(C_Int,&mut recog.err_handler)?;

					}
				}

			C_Long 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(459);
					recog.base.match_token(C_Long,&mut recog.err_handler)?;

					}
				}

			C_Float 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(460);
					recog.base.match_token(C_Float,&mut recog.err_handler)?;

					}
				}

			C_Double 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(461);
					recog.base.match_token(C_Double,&mut recog.err_handler)?;

					}
				}

			C_Signed 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					recog.base.set_state(462);
					recog.base.match_token(C_Signed,&mut recog.err_handler)?;

					}
				}

			C_Unsigned 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					recog.base.set_state(463);
					recog.base.match_token(C_Unsigned,&mut recog.err_handler)?;

					}
				}

			C_Bool 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					recog.base.set_state(464);
					recog.base.match_token(C_Bool,&mut recog.err_handler)?;

					}
				}

			C_Complex 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					recog.base.set_state(465);
					recog.base.match_token(C_Complex,&mut recog.err_handler)?;

					}
				}

			C_T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					recog.base.set_state(466);
					recog.base.match_token(C_T__3,&mut recog.err_handler)?;

					}
				}

			C_T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13)?;
					recog.base.enter_outer_alt(None, 13)?;
					{
					recog.base.set_state(467);
					recog.base.match_token(C_T__4,&mut recog.err_handler)?;

					}
				}

			C_T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 14)?;
					recog.base.enter_outer_alt(None, 14)?;
					{
					recog.base.set_state(468);
					recog.base.match_token(C_T__5,&mut recog.err_handler)?;

					}
				}

			C_T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 15)?;
					recog.base.enter_outer_alt(None, 15)?;
					{
					recog.base.set_state(469);
					recog.base.match_token(C_T__0,&mut recog.err_handler)?;

					recog.base.set_state(470);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					recog.base.set_state(471);
					_la = recog.base.input.la(1);
					if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 112) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(472);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

			C_Atomic 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 16)?;
					recog.base.enter_outer_alt(None, 16)?;
					{
					/*InvokeRule atomicTypeSpecifier*/
					recog.base.set_state(473);
					recog.atomicTypeSpecifier()?;

					}
				}

			C_Struct |C_Union 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 17)?;
					recog.base.enter_outer_alt(None, 17)?;
					{
					/*InvokeRule structOrUnionSpecifier*/
					recog.base.set_state(474);
					recog.structOrUnionSpecifier()?;

					}
				}

			C_Enum 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 18)?;
					recog.base.enter_outer_alt(None, 18)?;
					{
					/*InvokeRule enumSpecifier*/
					recog.base.set_state(475);
					recog.enumSpecifier()?;

					}
				}

			C_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 19)?;
					recog.base.enter_outer_alt(None, 19)?;
					{
					/*InvokeRule typedefName*/
					recog.base.set_state(476);
					recog.typedefName()?;

					}
				}

			C_T__6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 20)?;
					recog.base.enter_outer_alt(None, 20)?;
					{
					recog.base.set_state(477);
					recog.base.match_token(C_T__6,&mut recog.err_handler)?;

					recog.base.set_state(478);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule constantExpression*/
					recog.base.set_state(479);
					recog.constantExpression()?;

					recog.base.set_state(480);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structOrUnionSpecifier ----------------
pub type StructOrUnionSpecifierContextAll<'input> = StructOrUnionSpecifierContext<'input>;


pub type StructOrUnionSpecifierContext<'input> = BaseParserRuleContext<'input,StructOrUnionSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct StructOrUnionSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StructOrUnionSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StructOrUnionSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structOrUnionSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structOrUnionSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructOrUnionSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structOrUnionSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structOrUnionSpecifier }
}
antlr4rust::tid!{StructOrUnionSpecifierContextExt<'a>}

impl<'input> StructOrUnionSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructOrUnionSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructOrUnionSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructOrUnionSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<StructOrUnionSpecifierContextExt<'input>>{

fn structOrUnion(&self) -> Option<Rc<StructOrUnionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBrace, 0)
}
fn structDeclarationList(&self) -> Option<Rc<StructDeclarationListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}

}

impl<'input> StructOrUnionSpecifierContextAttrs<'input> for StructOrUnionSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structOrUnionSpecifier(&mut self,)
	-> Result<Rc<StructOrUnionSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructOrUnionSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_structOrUnionSpecifier);
        let mut _localctx: Rc<StructOrUnionSpecifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(495);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule structOrUnion*/
					recog.base.set_state(484);
					recog.structOrUnion()?;

					recog.base.set_state(486);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Identifier {
						{
						recog.base.set_state(485);
						recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(488);
					recog.base.match_token(C_LeftBrace,&mut recog.err_handler)?;

					/*InvokeRule structDeclarationList*/
					recog.base.set_state(489);
					recog.structDeclarationList()?;

					recog.base.set_state(490);
					recog.base.match_token(C_RightBrace,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule structOrUnion*/
					recog.base.set_state(492);
					recog.structOrUnion()?;

					recog.base.set_state(493);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structOrUnion ----------------
pub type StructOrUnionContextAll<'input> = StructOrUnionContext<'input>;


pub type StructOrUnionContext<'input> = BaseParserRuleContext<'input,StructOrUnionContextExt<'input>>;

#[derive(Clone)]
pub struct StructOrUnionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StructOrUnionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StructOrUnionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structOrUnion(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structOrUnion(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructOrUnionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structOrUnion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structOrUnion }
}
antlr4rust::tid!{StructOrUnionContextExt<'a>}

impl<'input> StructOrUnionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructOrUnionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructOrUnionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructOrUnionContextAttrs<'input>: CParserContext<'input> + BorrowMut<StructOrUnionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Struct
/// Returns `None` if there is no child corresponding to token Struct
fn Struct(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Struct, 0)
}
/// Retrieves first TerminalNode corresponding to token Union
/// Returns `None` if there is no child corresponding to token Union
fn Union(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Union, 0)
}

}

impl<'input> StructOrUnionContextAttrs<'input> for StructOrUnionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structOrUnion(&mut self,)
	-> Result<Rc<StructOrUnionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructOrUnionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_structOrUnion);
        let mut _localctx: Rc<StructOrUnionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(497);
			_la = recog.base.input.la(1);
			if { !(_la==C_Struct || _la==C_Union) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structDeclarationList ----------------
pub type StructDeclarationListContextAll<'input> = StructDeclarationListContext<'input>;


pub type StructDeclarationListContext<'input> = BaseParserRuleContext<'input,StructDeclarationListContextExt<'input>>;

#[derive(Clone)]
pub struct StructDeclarationListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StructDeclarationListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StructDeclarationListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structDeclarationList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structDeclarationList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructDeclarationListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDeclarationList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDeclarationList }
}
antlr4rust::tid!{StructDeclarationListContextExt<'a>}

impl<'input> StructDeclarationListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructDeclarationListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDeclarationListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructDeclarationListContextAttrs<'input>: CParserContext<'input> + BorrowMut<StructDeclarationListContextExt<'input>>{

fn structDeclaration_all(&self) ->  Vec<Rc<StructDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structDeclaration(&self, i: usize) -> Option<Rc<StructDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StructDeclarationListContextAttrs<'input> for StructDeclarationListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structDeclarationList(&mut self,)
	-> Result<Rc<StructDeclarationListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDeclarationListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_structDeclarationList);
        let mut _localctx: Rc<StructDeclarationListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(500); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule structDeclaration*/
				recog.base.set_state(499);
				recog.structDeclaration()?;

				}
				}
				recog.base.set_state(502); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 1367343346) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 1193168225) != 0) || _la==C_Identifier) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structDeclaration ----------------
pub type StructDeclarationContextAll<'input> = StructDeclarationContext<'input>;


pub type StructDeclarationContext<'input> = BaseParserRuleContext<'input,StructDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct StructDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StructDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StructDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structDeclaration(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDeclaration }
}
antlr4rust::tid!{StructDeclarationContextExt<'a>}

impl<'input> StructDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructDeclarationContextAttrs<'input>: CParserContext<'input> + BorrowMut<StructDeclarationContextExt<'input>>{

fn specifierQualifierList(&self) -> Option<Rc<SpecifierQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structDeclaratorList(&self) -> Option<Rc<StructDeclaratorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
fn staticAssertDeclaration(&self) -> Option<Rc<StaticAssertDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StructDeclarationContextAttrs<'input> for StructDeclarationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structDeclaration(&mut self,)
	-> Result<Rc<StructDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_structDeclaration);
        let mut _localctx: Rc<StructDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(512);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule specifierQualifierList*/
					recog.base.set_state(504);
					recog.specifierQualifierList()?;

					/*InvokeRule structDeclaratorList*/
					recog.base.set_state(505);
					recog.structDeclaratorList()?;

					recog.base.set_state(506);
					recog.base.match_token(C_Semi,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule specifierQualifierList*/
					recog.base.set_state(508);
					recog.specifierQualifierList()?;

					recog.base.set_state(509);
					recog.base.match_token(C_Semi,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule staticAssertDeclaration*/
					recog.base.set_state(511);
					recog.staticAssertDeclaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- specifierQualifierList ----------------
pub type SpecifierQualifierListContextAll<'input> = SpecifierQualifierListContext<'input>;


pub type SpecifierQualifierListContext<'input> = BaseParserRuleContext<'input,SpecifierQualifierListContextExt<'input>>;

#[derive(Clone)]
pub struct SpecifierQualifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for SpecifierQualifierListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for SpecifierQualifierListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_specifierQualifierList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_specifierQualifierList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SpecifierQualifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_specifierQualifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_specifierQualifierList }
}
antlr4rust::tid!{SpecifierQualifierListContextExt<'a>}

impl<'input> SpecifierQualifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SpecifierQualifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SpecifierQualifierListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SpecifierQualifierListContextAttrs<'input>: CParserContext<'input> + BorrowMut<SpecifierQualifierListContextExt<'input>>{

fn typeSpecifier(&self) -> Option<Rc<TypeSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeQualifier(&self) -> Option<Rc<TypeQualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn specifierQualifierList(&self) -> Option<Rc<SpecifierQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SpecifierQualifierListContextAttrs<'input> for SpecifierQualifierListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn specifierQualifierList(&mut self,)
	-> Result<Rc<SpecifierQualifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SpecifierQualifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_specifierQualifierList);
        let mut _localctx: Rc<SpecifierQualifierListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(516);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(41,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule typeSpecifier*/
					recog.base.set_state(514);
					recog.typeSpecifier()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule typeQualifier*/
					recog.base.set_state(515);
					recog.typeQualifier()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(519);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(42,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule specifierQualifierList*/
					recog.base.set_state(518);
					recog.specifierQualifierList()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structDeclaratorList ----------------
pub type StructDeclaratorListContextAll<'input> = StructDeclaratorListContext<'input>;


pub type StructDeclaratorListContext<'input> = BaseParserRuleContext<'input,StructDeclaratorListContextExt<'input>>;

#[derive(Clone)]
pub struct StructDeclaratorListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StructDeclaratorListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StructDeclaratorListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structDeclaratorList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structDeclaratorList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructDeclaratorListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDeclaratorList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDeclaratorList }
}
antlr4rust::tid!{StructDeclaratorListContextExt<'a>}

impl<'input> StructDeclaratorListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructDeclaratorListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDeclaratorListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructDeclaratorListContextAttrs<'input>: CParserContext<'input> + BorrowMut<StructDeclaratorListContextExt<'input>>{

fn structDeclarator_all(&self) ->  Vec<Rc<StructDeclaratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structDeclarator(&self, i: usize) -> Option<Rc<StructDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> StructDeclaratorListContextAttrs<'input> for StructDeclaratorListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structDeclaratorList(&mut self,)
	-> Result<Rc<StructDeclaratorListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDeclaratorListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_structDeclaratorList);
        let mut _localctx: Rc<StructDeclaratorListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule structDeclarator*/
			recog.base.set_state(521);
			recog.structDeclarator()?;

			recog.base.set_state(526);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(522);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				/*InvokeRule structDeclarator*/
				recog.base.set_state(523);
				recog.structDeclarator()?;

				}
				}
				recog.base.set_state(528);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structDeclarator ----------------
pub type StructDeclaratorContextAll<'input> = StructDeclaratorContext<'input>;


pub type StructDeclaratorContext<'input> = BaseParserRuleContext<'input,StructDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct StructDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StructDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StructDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structDeclarator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structDeclarator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDeclarator }
}
antlr4rust::tid!{StructDeclaratorContextExt<'a>}

impl<'input> StructDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDeclaratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructDeclaratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<StructDeclaratorContextExt<'input>>{

fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Colon, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StructDeclaratorContextAttrs<'input> for StructDeclaratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structDeclarator(&mut self,)
	-> Result<Rc<StructDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_structDeclarator);
        let mut _localctx: Rc<StructDeclaratorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(535);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(45,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule declarator*/
					recog.base.set_state(529);
					recog.declarator()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(531);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 64000) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 8454145) != 0) || _la==C_Identifier {
						{
						/*InvokeRule declarator*/
						recog.base.set_state(530);
						recog.declarator()?;

						}
					}

					recog.base.set_state(533);
					recog.base.match_token(C_Colon,&mut recog.err_handler)?;

					/*InvokeRule constantExpression*/
					recog.base.set_state(534);
					recog.constantExpression()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumSpecifier ----------------
pub type EnumSpecifierContextAll<'input> = EnumSpecifierContext<'input>;


pub type EnumSpecifierContext<'input> = BaseParserRuleContext<'input,EnumSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct EnumSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for EnumSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for EnumSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EnumSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumSpecifier }
}
antlr4rust::tid!{EnumSpecifierContextExt<'a>}

impl<'input> EnumSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<EnumSpecifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Enum
/// Returns `None` if there is no child corresponding to token Enum
fn Enum(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Enum, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBrace, 0)
}
fn enumeratorList(&self) -> Option<Rc<EnumeratorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}

}

impl<'input> EnumSpecifierContextAttrs<'input> for EnumSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumSpecifier(&mut self,)
	-> Result<Rc<EnumSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_enumSpecifier);
        let mut _localctx: Rc<EnumSpecifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(550);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(537);
					recog.base.match_token(C_Enum,&mut recog.err_handler)?;

					recog.base.set_state(539);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Identifier {
						{
						recog.base.set_state(538);
						recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(541);
					recog.base.match_token(C_LeftBrace,&mut recog.err_handler)?;

					/*InvokeRule enumeratorList*/
					recog.base.set_state(542);
					recog.enumeratorList()?;

					recog.base.set_state(544);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Comma {
						{
						recog.base.set_state(543);
						recog.base.match_token(C_Comma,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(546);
					recog.base.match_token(C_RightBrace,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(548);
					recog.base.match_token(C_Enum,&mut recog.err_handler)?;

					recog.base.set_state(549);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumeratorList ----------------
pub type EnumeratorListContextAll<'input> = EnumeratorListContext<'input>;


pub type EnumeratorListContext<'input> = BaseParserRuleContext<'input,EnumeratorListContextExt<'input>>;

#[derive(Clone)]
pub struct EnumeratorListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for EnumeratorListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for EnumeratorListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumeratorList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumeratorList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EnumeratorListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumeratorList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumeratorList }
}
antlr4rust::tid!{EnumeratorListContextExt<'a>}

impl<'input> EnumeratorListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumeratorListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumeratorListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumeratorListContextAttrs<'input>: CParserContext<'input> + BorrowMut<EnumeratorListContextExt<'input>>{

fn enumerator_all(&self) ->  Vec<Rc<EnumeratorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumerator(&self, i: usize) -> Option<Rc<EnumeratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> EnumeratorListContextAttrs<'input> for EnumeratorListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumeratorList(&mut self,)
	-> Result<Rc<EnumeratorListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumeratorListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_enumeratorList);
        let mut _localctx: Rc<EnumeratorListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule enumerator*/
			recog.base.set_state(552);
			recog.enumerator()?;

			recog.base.set_state(557);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(49,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(553);
					recog.base.match_token(C_Comma,&mut recog.err_handler)?;

					/*InvokeRule enumerator*/
					recog.base.set_state(554);
					recog.enumerator()?;

					}
					} 
				}
				recog.base.set_state(559);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(49,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumerator ----------------
pub type EnumeratorContextAll<'input> = EnumeratorContext<'input>;


pub type EnumeratorContext<'input> = BaseParserRuleContext<'input,EnumeratorContextExt<'input>>;

#[derive(Clone)]
pub struct EnumeratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for EnumeratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for EnumeratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumerator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumerator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EnumeratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumerator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumerator }
}
antlr4rust::tid!{EnumeratorContextExt<'a>}

impl<'input> EnumeratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumeratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumeratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumeratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<EnumeratorContextExt<'input>>{

fn enumerationConstant(&self) -> Option<Rc<EnumerationConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Assign
/// Returns `None` if there is no child corresponding to token Assign
fn Assign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Assign, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumeratorContextAttrs<'input> for EnumeratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumerator(&mut self,)
	-> Result<Rc<EnumeratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumeratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_enumerator);
        let mut _localctx: Rc<EnumeratorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule enumerationConstant*/
			recog.base.set_state(560);
			recog.enumerationConstant()?;

			recog.base.set_state(563);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_Assign {
				{
				recog.base.set_state(561);
				recog.base.match_token(C_Assign,&mut recog.err_handler)?;

				/*InvokeRule constantExpression*/
				recog.base.set_state(562);
				recog.constantExpression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumerationConstant ----------------
pub type EnumerationConstantContextAll<'input> = EnumerationConstantContext<'input>;


pub type EnumerationConstantContext<'input> = BaseParserRuleContext<'input,EnumerationConstantContextExt<'input>>;

#[derive(Clone)]
pub struct EnumerationConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for EnumerationConstantContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for EnumerationConstantContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumerationConstant(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumerationConstant(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EnumerationConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumerationConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumerationConstant }
}
antlr4rust::tid!{EnumerationConstantContextExt<'a>}

impl<'input> EnumerationConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumerationConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumerationConstantContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumerationConstantContextAttrs<'input>: CParserContext<'input> + BorrowMut<EnumerationConstantContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}

}

impl<'input> EnumerationConstantContextAttrs<'input> for EnumerationConstantContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumerationConstant(&mut self,)
	-> Result<Rc<EnumerationConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumerationConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_enumerationConstant);
        let mut _localctx: Rc<EnumerationConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(565);
			recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- atomicTypeSpecifier ----------------
pub type AtomicTypeSpecifierContextAll<'input> = AtomicTypeSpecifierContext<'input>;


pub type AtomicTypeSpecifierContext<'input> = BaseParserRuleContext<'input,AtomicTypeSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct AtomicTypeSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AtomicTypeSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AtomicTypeSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_atomicTypeSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_atomicTypeSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AtomicTypeSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomicTypeSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomicTypeSpecifier }
}
antlr4rust::tid!{AtomicTypeSpecifierContextExt<'a>}

impl<'input> AtomicTypeSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AtomicTypeSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomicTypeSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AtomicTypeSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<AtomicTypeSpecifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Atomic
/// Returns `None` if there is no child corresponding to token Atomic
fn Atomic(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Atomic, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}

}

impl<'input> AtomicTypeSpecifierContextAttrs<'input> for AtomicTypeSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn atomicTypeSpecifier(&mut self,)
	-> Result<Rc<AtomicTypeSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomicTypeSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_atomicTypeSpecifier);
        let mut _localctx: Rc<AtomicTypeSpecifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(567);
			recog.base.match_token(C_Atomic,&mut recog.err_handler)?;

			recog.base.set_state(568);
			recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

			/*InvokeRule typeName*/
			recog.base.set_state(569);
			recog.typeName()?;

			recog.base.set_state(570);
			recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- typeQualifier ----------------
pub type TypeQualifierContextAll<'input> = TypeQualifierContext<'input>;


pub type TypeQualifierContext<'input> = BaseParserRuleContext<'input,TypeQualifierContextExt<'input>>;

#[derive(Clone)]
pub struct TypeQualifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for TypeQualifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for TypeQualifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeQualifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeQualifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TypeQualifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeQualifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeQualifier }
}
antlr4rust::tid!{TypeQualifierContextExt<'a>}

impl<'input> TypeQualifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeQualifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeQualifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeQualifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<TypeQualifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Const
/// Returns `None` if there is no child corresponding to token Const
fn Const(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Const, 0)
}
/// Retrieves first TerminalNode corresponding to token Restrict
/// Returns `None` if there is no child corresponding to token Restrict
fn Restrict(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Restrict, 0)
}
/// Retrieves first TerminalNode corresponding to token Volatile
/// Returns `None` if there is no child corresponding to token Volatile
fn Volatile(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Volatile, 0)
}
/// Retrieves first TerminalNode corresponding to token Atomic
/// Returns `None` if there is no child corresponding to token Atomic
fn Atomic(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Atomic, 0)
}

}

impl<'input> TypeQualifierContextAttrs<'input> for TypeQualifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeQualifier(&mut self,)
	-> Result<Rc<TypeQualifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeQualifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_typeQualifier);
        let mut _localctx: Rc<TypeQualifierContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(572);
			_la = recog.base.input.la(1);
			if { !(_la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- functionSpecifier ----------------
pub type FunctionSpecifierContextAll<'input> = FunctionSpecifierContext<'input>;


pub type FunctionSpecifierContext<'input> = BaseParserRuleContext<'input,FunctionSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for FunctionSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for FunctionSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_functionSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_functionSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FunctionSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionSpecifier }
}
antlr4rust::tid!{FunctionSpecifierContextExt<'a>}

impl<'input> FunctionSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FunctionSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<FunctionSpecifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Inline
/// Returns `None` if there is no child corresponding to token Inline
fn Inline(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Inline, 0)
}
/// Retrieves first TerminalNode corresponding to token Noreturn
/// Returns `None` if there is no child corresponding to token Noreturn
fn Noreturn(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Noreturn, 0)
}
fn gccAttributeSpecifier(&self) -> Option<Rc<GccAttributeSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}

}

impl<'input> FunctionSpecifierContextAttrs<'input> for FunctionSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn functionSpecifier(&mut self,)
	-> Result<Rc<FunctionSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_functionSpecifier);
        let mut _localctx: Rc<FunctionSpecifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(583);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_Inline 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(574);
					recog.base.match_token(C_Inline,&mut recog.err_handler)?;

					}
				}

			C_Noreturn 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(575);
					recog.base.match_token(C_Noreturn,&mut recog.err_handler)?;

					}
				}

			C_T__7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(576);
					recog.base.match_token(C_T__7,&mut recog.err_handler)?;

					}
				}

			C_T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(577);
					recog.base.match_token(C_T__8,&mut recog.err_handler)?;

					}
				}

			C_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule gccAttributeSpecifier*/
					recog.base.set_state(578);
					recog.gccAttributeSpecifier()?;

					}
				}

			C_T__9 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(579);
					recog.base.match_token(C_T__9,&mut recog.err_handler)?;

					recog.base.set_state(580);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					recog.base.set_state(581);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(582);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- alignmentSpecifier ----------------
pub type AlignmentSpecifierContextAll<'input> = AlignmentSpecifierContext<'input>;


pub type AlignmentSpecifierContext<'input> = BaseParserRuleContext<'input,AlignmentSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct AlignmentSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AlignmentSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AlignmentSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_alignmentSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_alignmentSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AlignmentSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_alignmentSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_alignmentSpecifier }
}
antlr4rust::tid!{AlignmentSpecifierContextExt<'a>}

impl<'input> AlignmentSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AlignmentSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AlignmentSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AlignmentSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<AlignmentSpecifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Alignas
/// Returns `None` if there is no child corresponding to token Alignas
fn Alignas(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Alignas, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AlignmentSpecifierContextAttrs<'input> for AlignmentSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn alignmentSpecifier(&mut self,)
	-> Result<Rc<AlignmentSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AlignmentSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_alignmentSpecifier);
        let mut _localctx: Rc<AlignmentSpecifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(585);
			recog.base.match_token(C_Alignas,&mut recog.err_handler)?;

			recog.base.set_state(586);
			recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

			recog.base.set_state(589);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(52,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule typeName*/
					recog.base.set_state(587);
					recog.typeName()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule constantExpression*/
					recog.base.set_state(588);
					recog.constantExpression()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(591);
			recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declarator ----------------
pub type DeclaratorContextAll<'input> = DeclaratorContext<'input>;


pub type DeclaratorContext<'input> = BaseParserRuleContext<'input,DeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct DeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declarator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declarator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarator }
}
antlr4rust::tid!{DeclaratorContextExt<'a>}

impl<'input> DeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclaratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclaratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<DeclaratorContextExt<'input>>{

fn directDeclarator(&self) -> Option<Rc<DirectDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pointer(&self) -> Option<Rc<PointerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn gccDeclaratorExtension_all(&self) ->  Vec<Rc<GccDeclaratorExtensionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn gccDeclaratorExtension(&self, i: usize) -> Option<Rc<GccDeclaratorExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DeclaratorContextAttrs<'input> for DeclaratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declarator(&mut self,)
	-> Result<Rc<DeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_declarator);
        let mut _localctx: Rc<DeclaratorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(594);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_Star || _la==C_Caret {
				{
				/*InvokeRule pointer*/
				recog.base.set_state(593);
				recog.pointer()?;

				}
			}

			/*InvokeRule directDeclarator*/
			recog.base.set_state(596);
			recog.directDeclarator_rec(0)?;

			recog.base.set_state(600);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule gccDeclaratorExtension*/
					recog.base.set_state(597);
					recog.gccDeclaratorExtension()?;

					}
					} 
				}
				recog.base.set_state(602);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(54,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- directDeclarator ----------------
pub type DirectDeclaratorContextAll<'input> = DirectDeclaratorContext<'input>;


pub type DirectDeclaratorContext<'input> = BaseParserRuleContext<'input,DirectDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct DirectDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DirectDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DirectDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_directDeclarator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_directDeclarator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DirectDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_directDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_directDeclarator }
}
antlr4rust::tid!{DirectDeclaratorContextExt<'a>}

impl<'input> DirectDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DirectDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DirectDeclaratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DirectDeclaratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<DirectDeclaratorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Colon, 0)
}
/// Retrieves first TerminalNode corresponding to token DigitSequence
/// Returns `None` if there is no child corresponding to token DigitSequence
fn DigitSequence(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_DigitSequence, 0)
}
fn vcSpecificModifer(&self) -> Option<Rc<VcSpecificModiferContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn directDeclarator(&self) -> Option<Rc<DirectDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBracket, 0)
}
fn typeQualifierList(&self) -> Option<Rc<TypeQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Static
/// Returns `None` if there is no child corresponding to token Static
fn Static(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Static, 0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Star, 0)
}
fn parameterTypeList(&self) -> Option<Rc<ParameterTypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DirectDeclaratorContextAttrs<'input> for DirectDeclaratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  directDeclarator(&mut self,)
	-> Result<Rc<DirectDeclaratorContextAll<'input>>,ANTLRError> {
		self.directDeclarator_rec(0)
	}

	fn directDeclarator_rec(&mut self, _p: i32)
	-> Result<Rc<DirectDeclaratorContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = DirectDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 96, RULE_directDeclarator, _p);
	    let mut _localctx: Rc<DirectDeclaratorContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 96;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(620);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(604);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(605);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule declarator*/
					recog.base.set_state(606);
					recog.declarator()?;

					recog.base.set_state(607);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(609);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(610);
					recog.base.match_token(C_Colon,&mut recog.err_handler)?;

					recog.base.set_state(611);
					recog.base.match_token(C_DigitSequence,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule vcSpecificModifer*/
					recog.base.set_state(612);
					recog.vcSpecificModifer()?;

					recog.base.set_state(613);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(615);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule vcSpecificModifer*/
					recog.base.set_state(616);
					recog.vcSpecificModifer()?;

					/*InvokeRule declarator*/
					recog.base.set_state(617);
					recog.declarator()?;

					recog.base.set_state(618);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(667);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(665);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(61,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(622);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(623);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(625);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
								{
								/*InvokeRule typeQualifierList*/
								recog.base.set_state(624);
								recog.typeQualifierList()?;

								}
							}

							recog.base.set_state(628);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
								{
								/*InvokeRule assignmentExpression*/
								recog.base.set_state(627);
								recog.assignmentExpression()?;

								}
							}

							recog.base.set_state(630);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(631);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(632);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(633);
							recog.base.match_token(C_Static,&mut recog.err_handler)?;

							recog.base.set_state(635);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
								{
								/*InvokeRule typeQualifierList*/
								recog.base.set_state(634);
								recog.typeQualifierList()?;

								}
							}

							/*InvokeRule assignmentExpression*/
							recog.base.set_state(637);
							recog.assignmentExpression()?;

							recog.base.set_state(638);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(640);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(641);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							/*InvokeRule typeQualifierList*/
							recog.base.set_state(642);
							recog.typeQualifierList()?;

							recog.base.set_state(643);
							recog.base.match_token(C_Static,&mut recog.err_handler)?;

							/*InvokeRule assignmentExpression*/
							recog.base.set_state(644);
							recog.assignmentExpression()?;

							recog.base.set_state(645);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(647);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(648);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(650);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
								{
								/*InvokeRule typeQualifierList*/
								recog.base.set_state(649);
								recog.typeQualifierList()?;

								}
							}

							recog.base.set_state(652);
							recog.base.match_token(C_Star,&mut recog.err_handler)?;

							recog.base.set_state(653);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(654);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(655);
							recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

							/*InvokeRule parameterTypeList*/
							recog.base.set_state(656);
							recog.parameterTypeList()?;

							recog.base.set_state(657);
							recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(659);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(660);
							recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

							recog.base.set_state(662);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==C_Identifier {
								{
								/*InvokeRule identifierList*/
								recog.base.set_state(661);
								recog.identifierList()?;

								}
							}

							recog.base.set_state(664);
							recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(669);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- vcSpecificModifer ----------------
pub type VcSpecificModiferContextAll<'input> = VcSpecificModiferContext<'input>;


pub type VcSpecificModiferContext<'input> = BaseParserRuleContext<'input,VcSpecificModiferContextExt<'input>>;

#[derive(Clone)]
pub struct VcSpecificModiferContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for VcSpecificModiferContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for VcSpecificModiferContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_vcSpecificModifer(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_vcSpecificModifer(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for VcSpecificModiferContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_vcSpecificModifer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_vcSpecificModifer }
}
antlr4rust::tid!{VcSpecificModiferContextExt<'a>}

impl<'input> VcSpecificModiferContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VcSpecificModiferContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VcSpecificModiferContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VcSpecificModiferContextAttrs<'input>: CParserContext<'input> + BorrowMut<VcSpecificModiferContextExt<'input>>{


}

impl<'input> VcSpecificModiferContextAttrs<'input> for VcSpecificModiferContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn vcSpecificModifer(&mut self,)
	-> Result<Rc<VcSpecificModiferContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VcSpecificModiferContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_vcSpecificModifer);
        let mut _localctx: Rc<VcSpecificModiferContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(670);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 64000) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- gccDeclaratorExtension ----------------
pub type GccDeclaratorExtensionContextAll<'input> = GccDeclaratorExtensionContext<'input>;


pub type GccDeclaratorExtensionContext<'input> = BaseParserRuleContext<'input,GccDeclaratorExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct GccDeclaratorExtensionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GccDeclaratorExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GccDeclaratorExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_gccDeclaratorExtension(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_gccDeclaratorExtension(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GccDeclaratorExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gccDeclaratorExtension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gccDeclaratorExtension }
}
antlr4rust::tid!{GccDeclaratorExtensionContextExt<'a>}

impl<'input> GccDeclaratorExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GccDeclaratorExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GccDeclaratorExtensionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GccDeclaratorExtensionContextAttrs<'input>: CParserContext<'input> + BorrowMut<GccDeclaratorExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token StringLiteral in current rule
fn StringLiteral_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token StringLiteral, starting from 0.
/// Returns `None` if number of children corresponding to token StringLiteral is less or equal than `i`.
fn StringLiteral(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_StringLiteral, i)
}
fn gccAttributeSpecifier(&self) -> Option<Rc<GccAttributeSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GccDeclaratorExtensionContextAttrs<'input> for GccDeclaratorExtensionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn gccDeclaratorExtension(&mut self,)
	-> Result<Rc<GccDeclaratorExtensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GccDeclaratorExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_gccDeclaratorExtension);
        let mut _localctx: Rc<GccDeclaratorExtensionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(681);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_T__15 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(672);
					recog.base.match_token(C_T__15,&mut recog.err_handler)?;

					recog.base.set_state(673);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					recog.base.set_state(675); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(674);
						recog.base.match_token(C_StringLiteral,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(677); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==C_StringLiteral) {break}
					}
					recog.base.set_state(679);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					}
				}

			C_T__16 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule gccAttributeSpecifier*/
					recog.base.set_state(680);
					recog.gccAttributeSpecifier()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- gccAttributeSpecifier ----------------
pub type GccAttributeSpecifierContextAll<'input> = GccAttributeSpecifierContext<'input>;


pub type GccAttributeSpecifierContext<'input> = BaseParserRuleContext<'input,GccAttributeSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct GccAttributeSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GccAttributeSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GccAttributeSpecifierContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_gccAttributeSpecifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_gccAttributeSpecifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GccAttributeSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gccAttributeSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gccAttributeSpecifier }
}
antlr4rust::tid!{GccAttributeSpecifierContextExt<'a>}

impl<'input> GccAttributeSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GccAttributeSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GccAttributeSpecifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GccAttributeSpecifierContextAttrs<'input>: CParserContext<'input> + BorrowMut<GccAttributeSpecifierContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token LeftParen in current rule
fn LeftParen_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftParen, starting from 0.
/// Returns `None` if number of children corresponding to token LeftParen is less or equal than `i`.
fn LeftParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, i)
}
fn gccAttributeList(&self) -> Option<Rc<GccAttributeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token RightParen in current rule
fn RightParen_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightParen, starting from 0.
/// Returns `None` if number of children corresponding to token RightParen is less or equal than `i`.
fn RightParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, i)
}

}

impl<'input> GccAttributeSpecifierContextAttrs<'input> for GccAttributeSpecifierContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn gccAttributeSpecifier(&mut self,)
	-> Result<Rc<GccAttributeSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GccAttributeSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_gccAttributeSpecifier);
        let mut _localctx: Rc<GccAttributeSpecifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(683);
			recog.base.match_token(C_T__16,&mut recog.err_handler)?;

			recog.base.set_state(684);
			recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

			recog.base.set_state(685);
			recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

			/*InvokeRule gccAttributeList*/
			recog.base.set_state(686);
			recog.gccAttributeList()?;

			recog.base.set_state(687);
			recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

			recog.base.set_state(688);
			recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- gccAttributeList ----------------
pub type GccAttributeListContextAll<'input> = GccAttributeListContext<'input>;


pub type GccAttributeListContext<'input> = BaseParserRuleContext<'input,GccAttributeListContextExt<'input>>;

#[derive(Clone)]
pub struct GccAttributeListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GccAttributeListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GccAttributeListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_gccAttributeList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_gccAttributeList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GccAttributeListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gccAttributeList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gccAttributeList }
}
antlr4rust::tid!{GccAttributeListContextExt<'a>}

impl<'input> GccAttributeListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GccAttributeListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GccAttributeListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GccAttributeListContextAttrs<'input>: CParserContext<'input> + BorrowMut<GccAttributeListContextExt<'input>>{

fn gccAttribute_all(&self) ->  Vec<Rc<GccAttributeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn gccAttribute(&self, i: usize) -> Option<Rc<GccAttributeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> GccAttributeListContextAttrs<'input> for GccAttributeListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn gccAttributeList(&mut self,)
	-> Result<Rc<GccAttributeListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GccAttributeListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_gccAttributeList);
        let mut _localctx: Rc<GccAttributeListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(691);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294967294) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 4294967295) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & 4160749567) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & 8388607) != 0) {
				{
				/*InvokeRule gccAttribute*/
				recog.base.set_state(690);
				recog.gccAttribute()?;

				}
			}

			recog.base.set_state(699);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(693);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				recog.base.set_state(695);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4294967294) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 4294967295) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & 4160749567) != 0) || ((((_la - 98)) & !0x3f) == 0 && ((1usize << (_la - 98)) & 8388607) != 0) {
					{
					/*InvokeRule gccAttribute*/
					recog.base.set_state(694);
					recog.gccAttribute()?;

					}
				}

				}
				}
				recog.base.set_state(701);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- gccAttribute ----------------
pub type GccAttributeContextAll<'input> = GccAttributeContext<'input>;


pub type GccAttributeContext<'input> = BaseParserRuleContext<'input,GccAttributeContextExt<'input>>;

#[derive(Clone)]
pub struct GccAttributeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for GccAttributeContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for GccAttributeContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_gccAttribute(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_gccAttribute(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GccAttributeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_gccAttribute }
	//fn type_rule_index() -> usize where Self: Sized { RULE_gccAttribute }
}
antlr4rust::tid!{GccAttributeContextExt<'a>}

impl<'input> GccAttributeContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GccAttributeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GccAttributeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GccAttributeContextAttrs<'input>: CParserContext<'input> + BorrowMut<GccAttributeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LeftParen in current rule
fn LeftParen_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LeftParen, starting from 0.
/// Returns `None` if number of children corresponding to token LeftParen is less or equal than `i`.
fn LeftParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, i)
}
/// Retrieves all `TerminalNode`s corresponding to token RightParen in current rule
fn RightParen_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token RightParen, starting from 0.
/// Returns `None` if number of children corresponding to token RightParen is less or equal than `i`.
fn RightParen(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, i)
}
fn argumentExpressionList(&self) -> Option<Rc<ArgumentExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GccAttributeContextAttrs<'input> for GccAttributeContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn gccAttribute(&mut self,)
	-> Result<Rc<GccAttributeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GccAttributeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_gccAttribute);
        let mut _localctx: Rc<GccAttributeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(702);
			_la = recog.base.input.la(1);
			if { _la <= 0 || (((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 536870915) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(708);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_LeftParen {
				{
				recog.base.set_state(703);
				recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

				recog.base.set_state(705);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
					{
					/*InvokeRule argumentExpressionList*/
					recog.base.set_state(704);
					recog.argumentExpressionList()?;

					}
				}

				recog.base.set_state(707);
				recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- pointer ----------------
pub type PointerContextAll<'input> = PointerContext<'input>;


pub type PointerContext<'input> = BaseParserRuleContext<'input,PointerContextExt<'input>>;

#[derive(Clone)]
pub struct PointerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for PointerContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for PointerContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_pointer(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_pointer(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PointerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pointer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pointer }
}
antlr4rust::tid!{PointerContextExt<'a>}

impl<'input> PointerContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PointerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PointerContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PointerContextAttrs<'input>: CParserContext<'input> + BorrowMut<PointerContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Star in current rule
fn Star_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Star, starting from 0.
/// Returns `None` if number of children corresponding to token Star is less or equal than `i`.
fn Star(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Star, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Caret in current rule
fn Caret_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Caret, starting from 0.
/// Returns `None` if number of children corresponding to token Caret is less or equal than `i`.
fn Caret(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Caret, i)
}
fn typeQualifierList_all(&self) ->  Vec<Rc<TypeQualifierListContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeQualifierList(&self, i: usize) -> Option<Rc<TypeQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PointerContextAttrs<'input> for PointerContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn pointer(&mut self,)
	-> Result<Rc<PointerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PointerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_pointer);
        let mut _localctx: Rc<PointerContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(714); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(710);
				_la = recog.base.input.la(1);
				if { !(_la==C_Star || _la==C_Caret) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				recog.base.set_state(712);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
					{
					/*InvokeRule typeQualifierList*/
					recog.base.set_state(711);
					recog.typeQualifierList()?;

					}
				}

				}
				}
				recog.base.set_state(716); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==C_Star || _la==C_Caret) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- typeQualifierList ----------------
pub type TypeQualifierListContextAll<'input> = TypeQualifierListContext<'input>;


pub type TypeQualifierListContext<'input> = BaseParserRuleContext<'input,TypeQualifierListContextExt<'input>>;

#[derive(Clone)]
pub struct TypeQualifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for TypeQualifierListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for TypeQualifierListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeQualifierList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeQualifierList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TypeQualifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeQualifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeQualifierList }
}
antlr4rust::tid!{TypeQualifierListContextExt<'a>}

impl<'input> TypeQualifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeQualifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeQualifierListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeQualifierListContextAttrs<'input>: CParserContext<'input> + BorrowMut<TypeQualifierListContextExt<'input>>{

fn typeQualifier_all(&self) ->  Vec<Rc<TypeQualifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeQualifier(&self, i: usize) -> Option<Rc<TypeQualifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TypeQualifierListContextAttrs<'input> for TypeQualifierListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeQualifierList(&mut self,)
	-> Result<Rc<TypeQualifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeQualifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_typeQualifierList);
        let mut _localctx: Rc<TypeQualifierListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(719); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule typeQualifier*/
				recog.base.set_state(718);
				recog.typeQualifier()?;

				}
				}
				recog.base.set_state(721); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- parameterTypeList ----------------
pub type ParameterTypeListContextAll<'input> = ParameterTypeListContext<'input>;


pub type ParameterTypeListContext<'input> = BaseParserRuleContext<'input,ParameterTypeListContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterTypeListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ParameterTypeListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ParameterTypeListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameterTypeList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameterTypeList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ParameterTypeListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterTypeList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterTypeList }
}
antlr4rust::tid!{ParameterTypeListContextExt<'a>}

impl<'input> ParameterTypeListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterTypeListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterTypeListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterTypeListContextAttrs<'input>: CParserContext<'input> + BorrowMut<ParameterTypeListContextExt<'input>>{

fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}
/// Retrieves first TerminalNode corresponding to token Ellipsis
/// Returns `None` if there is no child corresponding to token Ellipsis
fn Ellipsis(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Ellipsis, 0)
}

}

impl<'input> ParameterTypeListContextAttrs<'input> for ParameterTypeListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterTypeList(&mut self,)
	-> Result<Rc<ParameterTypeListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterTypeListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_parameterTypeList);
        let mut _localctx: Rc<ParameterTypeListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule parameterList*/
			recog.base.set_state(723);
			recog.parameterList()?;

			recog.base.set_state(726);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_Comma {
				{
				recog.base.set_state(724);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				recog.base.set_state(725);
				recog.base.match_token(C_Ellipsis,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- parameterList ----------------
pub type ParameterListContextAll<'input> = ParameterListContext<'input>;


pub type ParameterListContext<'input> = BaseParserRuleContext<'input,ParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ParameterListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameterList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameterList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterList }
}
antlr4rust::tid!{ParameterListContextExt<'a>}

impl<'input> ParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterListContextAttrs<'input>: CParserContext<'input> + BorrowMut<ParameterListContextExt<'input>>{

fn parameterDeclaration_all(&self) ->  Vec<Rc<ParameterDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameterDeclaration(&self, i: usize) -> Option<Rc<ParameterDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> ParameterListContextAttrs<'input> for ParameterListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterList(&mut self,)
	-> Result<Rc<ParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_parameterList);
        let mut _localctx: Rc<ParameterListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule parameterDeclaration*/
			recog.base.set_state(728);
			recog.parameterDeclaration()?;

			recog.base.set_state(733);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(74,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(729);
					recog.base.match_token(C_Comma,&mut recog.err_handler)?;

					/*InvokeRule parameterDeclaration*/
					recog.base.set_state(730);
					recog.parameterDeclaration()?;

					}
					} 
				}
				recog.base.set_state(735);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(74,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- parameterDeclaration ----------------
pub type ParameterDeclarationContextAll<'input> = ParameterDeclarationContext<'input>;


pub type ParameterDeclarationContext<'input> = BaseParserRuleContext<'input,ParameterDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ParameterDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ParameterDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameterDeclaration(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameterDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ParameterDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterDeclaration }
}
antlr4rust::tid!{ParameterDeclarationContextExt<'a>}

impl<'input> ParameterDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterDeclarationContextAttrs<'input>: CParserContext<'input> + BorrowMut<ParameterDeclarationContextExt<'input>>{

fn declarationSpecifiers(&self) -> Option<Rc<DeclarationSpecifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declarationSpecifiers2(&self) -> Option<Rc<DeclarationSpecifiers2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn abstractDeclarator(&self) -> Option<Rc<AbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterDeclarationContextAttrs<'input> for ParameterDeclarationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterDeclaration(&mut self,)
	-> Result<Rc<ParameterDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_parameterDeclaration);
        let mut _localctx: Rc<ParameterDeclarationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(743);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(76,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule declarationSpecifiers*/
					recog.base.set_state(736);
					recog.declarationSpecifiers()?;

					/*InvokeRule declarator*/
					recog.base.set_state(737);
					recog.declarator()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule declarationSpecifiers2*/
					recog.base.set_state(739);
					recog.declarationSpecifiers2()?;

					recog.base.set_state(741);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 8454149) != 0) {
						{
						/*InvokeRule abstractDeclarator*/
						recog.base.set_state(740);
						recog.abstractDeclarator()?;

						}
					}

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- identifierList ----------------
pub type IdentifierListContextAll<'input> = IdentifierListContext<'input>;


pub type IdentifierListContext<'input> = BaseParserRuleContext<'input,IdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for IdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for IdentifierListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_identifierList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_identifierList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierList }
}
antlr4rust::tid!{IdentifierListContextExt<'a>}

impl<'input> IdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierListContextAttrs<'input>: CParserContext<'input> + BorrowMut<IdentifierListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> IdentifierListContextAttrs<'input> for IdentifierListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn identifierList(&mut self,)
	-> Result<Rc<IdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_identifierList);
        let mut _localctx: Rc<IdentifierListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(745);
			recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

			recog.base.set_state(750);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(746);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				recog.base.set_state(747);
				recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(752);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- typeName ----------------
pub type TypeNameContextAll<'input> = TypeNameContext<'input>;


pub type TypeNameContext<'input> = BaseParserRuleContext<'input,TypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct TypeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for TypeNameContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for TypeNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TypeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeName }
}
antlr4rust::tid!{TypeNameContextExt<'a>}

impl<'input> TypeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeNameContextAttrs<'input>: CParserContext<'input> + BorrowMut<TypeNameContextExt<'input>>{

fn specifierQualifierList(&self) -> Option<Rc<SpecifierQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn abstractDeclarator(&self) -> Option<Rc<AbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeNameContextAttrs<'input> for TypeNameContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeName(&mut self,)
	-> Result<Rc<TypeNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_typeName);
        let mut _localctx: Rc<TypeNameContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule specifierQualifierList*/
			recog.base.set_state(753);
			recog.specifierQualifierList()?;

			recog.base.set_state(755);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 8454149) != 0) {
				{
				/*InvokeRule abstractDeclarator*/
				recog.base.set_state(754);
				recog.abstractDeclarator()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- abstractDeclarator ----------------
pub type AbstractDeclaratorContextAll<'input> = AbstractDeclaratorContext<'input>;


pub type AbstractDeclaratorContext<'input> = BaseParserRuleContext<'input,AbstractDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct AbstractDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for AbstractDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for AbstractDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_abstractDeclarator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_abstractDeclarator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AbstractDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_abstractDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_abstractDeclarator }
}
antlr4rust::tid!{AbstractDeclaratorContextExt<'a>}

impl<'input> AbstractDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AbstractDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AbstractDeclaratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AbstractDeclaratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<AbstractDeclaratorContextExt<'input>>{

fn pointer(&self) -> Option<Rc<PointerContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn directAbstractDeclarator(&self) -> Option<Rc<DirectAbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn gccDeclaratorExtension_all(&self) ->  Vec<Rc<GccDeclaratorExtensionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn gccDeclaratorExtension(&self, i: usize) -> Option<Rc<GccDeclaratorExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AbstractDeclaratorContextAttrs<'input> for AbstractDeclaratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn abstractDeclarator(&mut self,)
	-> Result<Rc<AbstractDeclaratorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AbstractDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_abstractDeclarator);
        let mut _localctx: Rc<AbstractDeclaratorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(768);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(81,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule pointer*/
					recog.base.set_state(757);
					recog.pointer()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(759);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Star || _la==C_Caret {
						{
						/*InvokeRule pointer*/
						recog.base.set_state(758);
						recog.pointer()?;

						}
					}

					/*InvokeRule directAbstractDeclarator*/
					recog.base.set_state(761);
					recog.directAbstractDeclarator_rec(0)?;

					recog.base.set_state(765);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==C_T__15 || _la==C_T__16 {
						{
						{
						/*InvokeRule gccDeclaratorExtension*/
						recog.base.set_state(762);
						recog.gccDeclaratorExtension()?;

						}
						}
						recog.base.set_state(767);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- directAbstractDeclarator ----------------
pub type DirectAbstractDeclaratorContextAll<'input> = DirectAbstractDeclaratorContext<'input>;


pub type DirectAbstractDeclaratorContext<'input> = BaseParserRuleContext<'input,DirectAbstractDeclaratorContextExt<'input>>;

#[derive(Clone)]
pub struct DirectAbstractDeclaratorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DirectAbstractDeclaratorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DirectAbstractDeclaratorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_directAbstractDeclarator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_directAbstractDeclarator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DirectAbstractDeclaratorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_directAbstractDeclarator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_directAbstractDeclarator }
}
antlr4rust::tid!{DirectAbstractDeclaratorContextExt<'a>}

impl<'input> DirectAbstractDeclaratorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DirectAbstractDeclaratorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DirectAbstractDeclaratorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DirectAbstractDeclaratorContextAttrs<'input>: CParserContext<'input> + BorrowMut<DirectAbstractDeclaratorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn abstractDeclarator(&self) -> Option<Rc<AbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn gccDeclaratorExtension_all(&self) ->  Vec<Rc<GccDeclaratorExtensionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn gccDeclaratorExtension(&self, i: usize) -> Option<Rc<GccDeclaratorExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBracket, 0)
}
fn typeQualifierList(&self) -> Option<Rc<TypeQualifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Static
/// Returns `None` if there is no child corresponding to token Static
fn Static(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Static, 0)
}
/// Retrieves first TerminalNode corresponding to token Star
/// Returns `None` if there is no child corresponding to token Star
fn Star(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Star, 0)
}
fn parameterTypeList(&self) -> Option<Rc<ParameterTypeListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn directAbstractDeclarator(&self) -> Option<Rc<DirectAbstractDeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DirectAbstractDeclaratorContextAttrs<'input> for DirectAbstractDeclaratorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  directAbstractDeclarator(&mut self,)
	-> Result<Rc<DirectAbstractDeclaratorContextAll<'input>>,ANTLRError> {
		self.directAbstractDeclarator_rec(0)
	}

	fn directAbstractDeclarator_rec(&mut self, _p: i32)
	-> Result<Rc<DirectAbstractDeclaratorContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 124, RULE_directAbstractDeclarator, _p);
	    let mut _localctx: Rc<DirectAbstractDeclaratorContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 124;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(816);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(88,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(771);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule abstractDeclarator*/
					recog.base.set_state(772);
					recog.abstractDeclarator()?;

					recog.base.set_state(773);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					recog.base.set_state(777);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule gccDeclaratorExtension*/
							recog.base.set_state(774);
							recog.gccDeclaratorExtension()?;

							}
							} 
						}
						recog.base.set_state(779);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(82,&mut recog.base)?;
					}
					}
				}
			,
				2 =>{
					{
					recog.base.set_state(780);
					recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

					recog.base.set_state(782);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
						{
						/*InvokeRule typeQualifierList*/
						recog.base.set_state(781);
						recog.typeQualifierList()?;

						}
					}

					recog.base.set_state(785);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
						{
						/*InvokeRule assignmentExpression*/
						recog.base.set_state(784);
						recog.assignmentExpression()?;

						}
					}

					recog.base.set_state(787);
					recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(788);
					recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

					recog.base.set_state(789);
					recog.base.match_token(C_Static,&mut recog.err_handler)?;

					recog.base.set_state(791);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
						{
						/*InvokeRule typeQualifierList*/
						recog.base.set_state(790);
						recog.typeQualifierList()?;

						}
					}

					/*InvokeRule assignmentExpression*/
					recog.base.set_state(793);
					recog.assignmentExpression()?;

					recog.base.set_state(794);
					recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					recog.base.set_state(796);
					recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

					/*InvokeRule typeQualifierList*/
					recog.base.set_state(797);
					recog.typeQualifierList()?;

					recog.base.set_state(798);
					recog.base.match_token(C_Static,&mut recog.err_handler)?;

					/*InvokeRule assignmentExpression*/
					recog.base.set_state(799);
					recog.assignmentExpression()?;

					recog.base.set_state(800);
					recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(802);
					recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

					recog.base.set_state(803);
					recog.base.match_token(C_Star,&mut recog.err_handler)?;

					recog.base.set_state(804);
					recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					recog.base.set_state(805);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					recog.base.set_state(807);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 3516008434) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2808049137) != 0) || _la==C_Identifier {
						{
						/*InvokeRule parameterTypeList*/
						recog.base.set_state(806);
						recog.parameterTypeList()?;

						}
					}

					recog.base.set_state(809);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					recog.base.set_state(813);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(87,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule gccDeclaratorExtension*/
							recog.base.set_state(810);
							recog.gccDeclaratorExtension()?;

							}
							} 
						}
						recog.base.set_state(815);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(87,&mut recog.base)?;
					}
					}
				}

				_ => {}
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(861);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(95,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(859);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(94,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directAbstractDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(818);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(819);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(821);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
								{
								/*InvokeRule typeQualifierList*/
								recog.base.set_state(820);
								recog.typeQualifierList()?;

								}
							}

							recog.base.set_state(824);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
								{
								/*InvokeRule assignmentExpression*/
								recog.base.set_state(823);
								recog.assignmentExpression()?;

								}
							}

							recog.base.set_state(826);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directAbstractDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(827);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(828);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(829);
							recog.base.match_token(C_Static,&mut recog.err_handler)?;

							recog.base.set_state(831);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==C_Const || ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 69633) != 0) {
								{
								/*InvokeRule typeQualifierList*/
								recog.base.set_state(830);
								recog.typeQualifierList()?;

								}
							}

							/*InvokeRule assignmentExpression*/
							recog.base.set_state(833);
							recog.assignmentExpression()?;

							recog.base.set_state(834);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directAbstractDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(836);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(837);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							/*InvokeRule typeQualifierList*/
							recog.base.set_state(838);
							recog.typeQualifierList()?;

							recog.base.set_state(839);
							recog.base.match_token(C_Static,&mut recog.err_handler)?;

							/*InvokeRule assignmentExpression*/
							recog.base.set_state(840);
							recog.assignmentExpression()?;

							recog.base.set_state(841);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directAbstractDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(843);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(844);
							recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

							recog.base.set_state(845);
							recog.base.match_token(C_Star,&mut recog.err_handler)?;

							recog.base.set_state(846);
							recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = DirectAbstractDeclaratorContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_directAbstractDeclarator)?;
							_localctx = tmp;
							recog.base.set_state(847);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(848);
							recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

							recog.base.set_state(850);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if (((_la) & !0x3f) == 0 && ((1usize << _la) & 3516008434) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 2808049137) != 0) || _la==C_Identifier {
								{
								/*InvokeRule parameterTypeList*/
								recog.base.set_state(849);
								recog.parameterTypeList()?;

								}
							}

							recog.base.set_state(852);
							recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

							recog.base.set_state(856);
							recog.err_handler.sync(&mut recog.base)?;
							_alt = recog.interpreter.adaptive_predict(93,&mut recog.base)?;
							while { _alt!=2 && _alt!=INVALID_ALT } {
								if _alt==1 {
									{
									{
									/*InvokeRule gccDeclaratorExtension*/
									recog.base.set_state(853);
									recog.gccDeclaratorExtension()?;

									}
									} 
								}
								recog.base.set_state(858);
								recog.err_handler.sync(&mut recog.base)?;
								_alt = recog.interpreter.adaptive_predict(93,&mut recog.base)?;
							}
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(863);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(95,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- typedefName ----------------
pub type TypedefNameContextAll<'input> = TypedefNameContext<'input>;


pub type TypedefNameContext<'input> = BaseParserRuleContext<'input,TypedefNameContextExt<'input>>;

#[derive(Clone)]
pub struct TypedefNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for TypedefNameContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for TypedefNameContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typedefName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typedefName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TypedefNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typedefName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typedefName }
}
antlr4rust::tid!{TypedefNameContextExt<'a>}

impl<'input> TypedefNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypedefNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypedefNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypedefNameContextAttrs<'input>: CParserContext<'input> + BorrowMut<TypedefNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}

}

impl<'input> TypedefNameContextAttrs<'input> for TypedefNameContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typedefName(&mut self,)
	-> Result<Rc<TypedefNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypedefNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_typedefName);
        let mut _localctx: Rc<TypedefNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(864);
			recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- initializer ----------------
pub type InitializerContextAll<'input> = InitializerContext<'input>;


pub type InitializerContext<'input> = BaseParserRuleContext<'input,InitializerContextExt<'input>>;

#[derive(Clone)]
pub struct InitializerContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for InitializerContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for InitializerContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_initializer(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_initializer(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for InitializerContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initializer }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initializer }
}
antlr4rust::tid!{InitializerContextExt<'a>}

impl<'input> InitializerContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InitializerContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitializerContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InitializerContextAttrs<'input>: CParserContext<'input> + BorrowMut<InitializerContextExt<'input>>{

fn assignmentExpression(&self) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBrace, 0)
}
fn initializerList(&self) -> Option<Rc<InitializerListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}

}

impl<'input> InitializerContextAttrs<'input> for InitializerContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn initializer(&mut self,)
	-> Result<Rc<InitializerContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitializerContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_initializer);
        let mut _localctx: Rc<InitializerContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(874);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_T__0 |C_T__1 |C_T__2 |C_Sizeof |C_Alignof |C_Generic |C_LeftParen |
			C_Plus |C_PlusPlus |C_Minus |C_MinusMinus |C_Star |C_And |C_AndAnd |C_Not |
			C_Tilde |C_Identifier |C_Constant |C_DigitSequence |C_StringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule assignmentExpression*/
					recog.base.set_state(866);
					recog.assignmentExpression()?;

					}
				}

			C_LeftBrace 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(867);
					recog.base.match_token(C_LeftBrace,&mut recog.err_handler)?;

					/*InvokeRule initializerList*/
					recog.base.set_state(868);
					recog.initializerList()?;

					recog.base.set_state(870);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_Comma {
						{
						recog.base.set_state(869);
						recog.base.match_token(C_Comma,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(872);
					recog.base.match_token(C_RightBrace,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- initializerList ----------------
pub type InitializerListContextAll<'input> = InitializerListContext<'input>;


pub type InitializerListContext<'input> = BaseParserRuleContext<'input,InitializerListContextExt<'input>>;

#[derive(Clone)]
pub struct InitializerListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for InitializerListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for InitializerListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_initializerList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_initializerList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for InitializerListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_initializerList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_initializerList }
}
antlr4rust::tid!{InitializerListContextExt<'a>}

impl<'input> InitializerListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InitializerListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitializerListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InitializerListContextAttrs<'input>: CParserContext<'input> + BorrowMut<InitializerListContextExt<'input>>{

fn initializer_all(&self) ->  Vec<Rc<InitializerContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn initializer(&self, i: usize) -> Option<Rc<InitializerContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn designation_all(&self) ->  Vec<Rc<DesignationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn designation(&self, i: usize) -> Option<Rc<DesignationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> InitializerListContextAttrs<'input> for InitializerListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn initializerList(&mut self,)
	-> Result<Rc<InitializerListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitializerListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_initializerList);
        let mut _localctx: Rc<InitializerListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(877);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==C_LeftBracket || _la==C_Dot {
				{
				/*InvokeRule designation*/
				recog.base.set_state(876);
				recog.designation()?;

				}
			}

			/*InvokeRule initializer*/
			recog.base.set_state(879);
			recog.initializer()?;

			recog.base.set_state(887);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(100,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(880);
					recog.base.match_token(C_Comma,&mut recog.err_handler)?;

					recog.base.set_state(882);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==C_LeftBracket || _la==C_Dot {
						{
						/*InvokeRule designation*/
						recog.base.set_state(881);
						recog.designation()?;

						}
					}

					/*InvokeRule initializer*/
					recog.base.set_state(884);
					recog.initializer()?;

					}
					} 
				}
				recog.base.set_state(889);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(100,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- designation ----------------
pub type DesignationContextAll<'input> = DesignationContext<'input>;


pub type DesignationContext<'input> = BaseParserRuleContext<'input,DesignationContextExt<'input>>;

#[derive(Clone)]
pub struct DesignationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DesignationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DesignationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_designation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_designation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DesignationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designation }
}
antlr4rust::tid!{DesignationContextExt<'a>}

impl<'input> DesignationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DesignationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DesignationContextAttrs<'input>: CParserContext<'input> + BorrowMut<DesignationContextExt<'input>>{

fn designatorList(&self) -> Option<Rc<DesignatorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Assign
/// Returns `None` if there is no child corresponding to token Assign
fn Assign(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Assign, 0)
}

}

impl<'input> DesignationContextAttrs<'input> for DesignationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn designation(&mut self,)
	-> Result<Rc<DesignationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_designation);
        let mut _localctx: Rc<DesignationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule designatorList*/
			recog.base.set_state(890);
			recog.designatorList()?;

			recog.base.set_state(891);
			recog.base.match_token(C_Assign,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- designatorList ----------------
pub type DesignatorListContextAll<'input> = DesignatorListContext<'input>;


pub type DesignatorListContext<'input> = BaseParserRuleContext<'input,DesignatorListContextExt<'input>>;

#[derive(Clone)]
pub struct DesignatorListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DesignatorListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DesignatorListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_designatorList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_designatorList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DesignatorListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designatorList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designatorList }
}
antlr4rust::tid!{DesignatorListContextExt<'a>}

impl<'input> DesignatorListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DesignatorListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignatorListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DesignatorListContextAttrs<'input>: CParserContext<'input> + BorrowMut<DesignatorListContextExt<'input>>{

fn designator_all(&self) ->  Vec<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn designator(&self, i: usize) -> Option<Rc<DesignatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DesignatorListContextAttrs<'input> for DesignatorListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn designatorList(&mut self,)
	-> Result<Rc<DesignatorListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignatorListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_designatorList);
        let mut _localctx: Rc<DesignatorListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(894); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule designator*/
				recog.base.set_state(893);
				recog.designator()?;

				}
				}
				recog.base.set_state(896); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==C_LeftBracket || _la==C_Dot) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- designator ----------------
pub type DesignatorContextAll<'input> = DesignatorContext<'input>;


pub type DesignatorContext<'input> = BaseParserRuleContext<'input,DesignatorContextExt<'input>>;

#[derive(Clone)]
pub struct DesignatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DesignatorContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DesignatorContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_designator(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_designator(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DesignatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_designator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_designator }
}
antlr4rust::tid!{DesignatorContextExt<'a>}

impl<'input> DesignatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DesignatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DesignatorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DesignatorContextAttrs<'input>: CParserContext<'input> + BorrowMut<DesignatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftBracket
/// Returns `None` if there is no child corresponding to token LeftBracket
fn LeftBracket(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBracket, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightBracket
/// Returns `None` if there is no child corresponding to token RightBracket
fn RightBracket(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Dot
/// Returns `None` if there is no child corresponding to token Dot
fn Dot(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Dot, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}

}

impl<'input> DesignatorContextAttrs<'input> for DesignatorContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn designator(&mut self,)
	-> Result<Rc<DesignatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DesignatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_designator);
        let mut _localctx: Rc<DesignatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(904);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_LeftBracket 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(898);
					recog.base.match_token(C_LeftBracket,&mut recog.err_handler)?;

					/*InvokeRule constantExpression*/
					recog.base.set_state(899);
					recog.constantExpression()?;

					recog.base.set_state(900);
					recog.base.match_token(C_RightBracket,&mut recog.err_handler)?;

					}
				}

			C_Dot 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(902);
					recog.base.match_token(C_Dot,&mut recog.err_handler)?;

					recog.base.set_state(903);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- staticAssertDeclaration ----------------
pub type StaticAssertDeclarationContextAll<'input> = StaticAssertDeclarationContext<'input>;


pub type StaticAssertDeclarationContext<'input> = BaseParserRuleContext<'input,StaticAssertDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct StaticAssertDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StaticAssertDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StaticAssertDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_staticAssertDeclaration(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_staticAssertDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StaticAssertDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_staticAssertDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_staticAssertDeclaration }
}
antlr4rust::tid!{StaticAssertDeclarationContextExt<'a>}

impl<'input> StaticAssertDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StaticAssertDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StaticAssertDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StaticAssertDeclarationContextAttrs<'input>: CParserContext<'input> + BorrowMut<StaticAssertDeclarationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StaticAssert
/// Returns `None` if there is no child corresponding to token StaticAssert
fn StaticAssert(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_StaticAssert, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token StringLiteral in current rule
fn StringLiteral_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token StringLiteral, starting from 0.
/// Returns `None` if number of children corresponding to token StringLiteral is less or equal than `i`.
fn StringLiteral(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_StringLiteral, i)
}

}

impl<'input> StaticAssertDeclarationContextAttrs<'input> for StaticAssertDeclarationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn staticAssertDeclaration(&mut self,)
	-> Result<Rc<StaticAssertDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StaticAssertDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_staticAssertDeclaration);
        let mut _localctx: Rc<StaticAssertDeclarationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(906);
			recog.base.match_token(C_StaticAssert,&mut recog.err_handler)?;

			recog.base.set_state(907);
			recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

			/*InvokeRule constantExpression*/
			recog.base.set_state(908);
			recog.constantExpression()?;

			recog.base.set_state(909);
			recog.base.match_token(C_Comma,&mut recog.err_handler)?;

			recog.base.set_state(911); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				recog.base.set_state(910);
				recog.base.match_token(C_StringLiteral,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(913); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==C_StringLiteral) {break}
			}
			recog.base.set_state(915);
			recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

			recog.base.set_state(916);
			recog.base.match_token(C_Semi,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_statement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr4rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn labeledStatement(&self) -> Option<Rc<LabeledStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionStatement(&self) -> Option<Rc<ExpressionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn selectionStatement(&self) -> Option<Rc<SelectionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iterationStatement(&self) -> Option<Rc<IterationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn jumpStatement(&self) -> Option<Rc<JumpStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
/// Retrieves first TerminalNode corresponding to token Volatile
/// Returns `None` if there is no child corresponding to token Volatile
fn Volatile(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Volatile, 0)
}
fn logicalOrExpression_all(&self) ->  Vec<Rc<LogicalOrExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logicalOrExpression(&self, i: usize) -> Option<Rc<LogicalOrExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Colon in current rule
fn Colon_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Colon, starting from 0.
/// Returns `None` if number of children corresponding to token Colon is less or equal than `i`.
fn Colon(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Colon, i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(955);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(109,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule labeledStatement*/
					recog.base.set_state(918);
					recog.labeledStatement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule compoundStatement*/
					recog.base.set_state(919);
					recog.compoundStatement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule expressionStatement*/
					recog.base.set_state(920);
					recog.expressionStatement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule selectionStatement*/
					recog.base.set_state(921);
					recog.selectionStatement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule iterationStatement*/
					recog.base.set_state(922);
					recog.iterationStatement()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule jumpStatement*/
					recog.base.set_state(923);
					recog.jumpStatement()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(924);
					_la = recog.base.input.la(1);
					if { !(_la==C_T__15 || _la==C_T__17) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(925);
					_la = recog.base.input.la(1);
					if { !(_la==C_T__18 || _la==C_Volatile) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(926);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					recog.base.set_state(935);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
						{
						/*InvokeRule logicalOrExpression*/
						recog.base.set_state(927);
						recog.logicalOrExpression()?;

						recog.base.set_state(932);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==C_Comma {
							{
							{
							recog.base.set_state(928);
							recog.base.match_token(C_Comma,&mut recog.err_handler)?;

							/*InvokeRule logicalOrExpression*/
							recog.base.set_state(929);
							recog.logicalOrExpression()?;

							}
							}
							recog.base.set_state(934);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(950);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==C_Colon {
						{
						{
						recog.base.set_state(937);
						recog.base.match_token(C_Colon,&mut recog.err_handler)?;

						recog.base.set_state(946);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
							{
							/*InvokeRule logicalOrExpression*/
							recog.base.set_state(938);
							recog.logicalOrExpression()?;

							recog.base.set_state(943);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==C_Comma {
								{
								{
								recog.base.set_state(939);
								recog.base.match_token(C_Comma,&mut recog.err_handler)?;

								/*InvokeRule logicalOrExpression*/
								recog.base.set_state(940);
								recog.logicalOrExpression()?;

								}
								}
								recog.base.set_state(945);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							}
						}

						}
						}
						recog.base.set_state(952);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(953);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					recog.base.set_state(954);
					recog.base.match_token(C_Semi,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- labeledStatement ----------------
pub type LabeledStatementContextAll<'input> = LabeledStatementContext<'input>;


pub type LabeledStatementContext<'input> = BaseParserRuleContext<'input,LabeledStatementContextExt<'input>>;

#[derive(Clone)]
pub struct LabeledStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for LabeledStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for LabeledStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_labeledStatement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_labeledStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LabeledStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_labeledStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_labeledStatement }
}
antlr4rust::tid!{LabeledStatementContextExt<'a>}

impl<'input> LabeledStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LabeledStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabeledStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LabeledStatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<LabeledStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Colon, 0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Case
/// Returns `None` if there is no child corresponding to token Case
fn Case(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Case, 0)
}
fn constantExpression(&self) -> Option<Rc<ConstantExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Default
/// Returns `None` if there is no child corresponding to token Default
fn Default(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Default, 0)
}

}

impl<'input> LabeledStatementContextAttrs<'input> for LabeledStatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn labeledStatement(&mut self,)
	-> Result<Rc<LabeledStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabeledStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_labeledStatement);
        let mut _localctx: Rc<LabeledStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(970);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(957);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(958);
					recog.base.match_token(C_Colon,&mut recog.err_handler)?;

					recog.base.set_state(960);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(110,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule statement*/
							recog.base.set_state(959);
							recog.statement()?;

							}
						}

						_ => {}
					}
					}
				}

			C_Case 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(962);
					recog.base.match_token(C_Case,&mut recog.err_handler)?;

					/*InvokeRule constantExpression*/
					recog.base.set_state(963);
					recog.constantExpression()?;

					recog.base.set_state(964);
					recog.base.match_token(C_Colon,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(965);
					recog.statement()?;

					}
				}

			C_Default 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(967);
					recog.base.match_token(C_Default,&mut recog.err_handler)?;

					recog.base.set_state(968);
					recog.base.match_token(C_Colon,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(969);
					recog.statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- compoundStatement ----------------
pub type CompoundStatementContextAll<'input> = CompoundStatementContext<'input>;


pub type CompoundStatementContext<'input> = BaseParserRuleContext<'input,CompoundStatementContextExt<'input>>;

#[derive(Clone)]
pub struct CompoundStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for CompoundStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for CompoundStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_compoundStatement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_compoundStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for CompoundStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compoundStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compoundStatement }
}
antlr4rust::tid!{CompoundStatementContextExt<'a>}

impl<'input> CompoundStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CompoundStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompoundStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CompoundStatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<CompoundStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LeftBrace
/// Returns `None` if there is no child corresponding to token LeftBrace
fn LeftBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token RightBrace
/// Returns `None` if there is no child corresponding to token RightBrace
fn RightBrace(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightBrace, 0)
}
fn blockItemList(&self) -> Option<Rc<BlockItemListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompoundStatementContextAttrs<'input> for CompoundStatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn compoundStatement(&mut self,)
	-> Result<Rc<CompoundStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompoundStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_compoundStatement);
        let mut _localctx: Rc<CompoundStatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(972);
			recog.base.match_token(C_LeftBrace,&mut recog.err_handler)?;

			recog.base.set_state(974);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 3757508606) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 4026531839) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 321515537) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
				{
				/*InvokeRule blockItemList*/
				recog.base.set_state(973);
				recog.blockItemList()?;

				}
			}

			recog.base.set_state(976);
			recog.base.match_token(C_RightBrace,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- blockItemList ----------------
pub type BlockItemListContextAll<'input> = BlockItemListContext<'input>;


pub type BlockItemListContext<'input> = BaseParserRuleContext<'input,BlockItemListContextExt<'input>>;

#[derive(Clone)]
pub struct BlockItemListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for BlockItemListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for BlockItemListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_blockItemList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_blockItemList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for BlockItemListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockItemList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockItemList }
}
antlr4rust::tid!{BlockItemListContextExt<'a>}

impl<'input> BlockItemListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BlockItemListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockItemListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BlockItemListContextAttrs<'input>: CParserContext<'input> + BorrowMut<BlockItemListContextExt<'input>>{

fn blockItem_all(&self) ->  Vec<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn blockItem(&self, i: usize) -> Option<Rc<BlockItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockItemListContextAttrs<'input> for BlockItemListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn blockItemList(&mut self,)
	-> Result<Rc<BlockItemListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockItemListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_blockItemList);
        let mut _localctx: Rc<BlockItemListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(979); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule blockItem*/
				recog.base.set_state(978);
				recog.blockItem()?;

				}
				}
				recog.base.set_state(981); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 3757508606) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 4026531839) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 321515537) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- blockItem ----------------
pub type BlockItemContextAll<'input> = BlockItemContext<'input>;


pub type BlockItemContext<'input> = BaseParserRuleContext<'input,BlockItemContextExt<'input>>;

#[derive(Clone)]
pub struct BlockItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for BlockItemContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for BlockItemContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_blockItem(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_blockItem(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for BlockItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_blockItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_blockItem }
}
antlr4rust::tid!{BlockItemContextExt<'a>}

impl<'input> BlockItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BlockItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockItemContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BlockItemContextAttrs<'input>: CParserContext<'input> + BorrowMut<BlockItemContextExt<'input>>{

fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BlockItemContextAttrs<'input> for BlockItemContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn blockItem(&mut self,)
	-> Result<Rc<BlockItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_blockItem);
        let mut _localctx: Rc<BlockItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(985);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(114,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule statement*/
					recog.base.set_state(983);
					recog.statement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule declaration*/
					recog.base.set_state(984);
					recog.declaration()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- expressionStatement ----------------
pub type ExpressionStatementContextAll<'input> = ExpressionStatementContext<'input>;


pub type ExpressionStatementContext<'input> = BaseParserRuleContext<'input,ExpressionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ExpressionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ExpressionStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_expressionStatement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_expressionStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionStatement }
}
antlr4rust::tid!{ExpressionStatementContextExt<'a>}

impl<'input> ExpressionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpressionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionStatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<ExpressionStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionStatementContextAttrs<'input> for ExpressionStatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn expressionStatement(&mut self,)
	-> Result<Rc<ExpressionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_expressionStatement);
        let mut _localctx: Rc<ExpressionStatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(988);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(987);
				recog.expression()?;

				}
			}

			recog.base.set_state(990);
			recog.base.match_token(C_Semi,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- selectionStatement ----------------
pub type SelectionStatementContextAll<'input> = SelectionStatementContext<'input>;


pub type SelectionStatementContext<'input> = BaseParserRuleContext<'input,SelectionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct SelectionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for SelectionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for SelectionStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_selectionStatement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_selectionStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SelectionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_selectionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_selectionStatement }
}
antlr4rust::tid!{SelectionStatementContextExt<'a>}

impl<'input> SelectionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SelectionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SelectionStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SelectionStatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<SelectionStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token If
/// Returns `None` if there is no child corresponding to token If
fn If(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_If, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Else
/// Returns `None` if there is no child corresponding to token Else
fn Else(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Else, 0)
}
/// Retrieves first TerminalNode corresponding to token Switch
/// Returns `None` if there is no child corresponding to token Switch
fn Switch(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Switch, 0)
}

}

impl<'input> SelectionStatementContextAttrs<'input> for SelectionStatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn selectionStatement(&mut self,)
	-> Result<Rc<SelectionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SelectionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_selectionStatement);
        let mut _localctx: Rc<SelectionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1007);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_If 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(992);
					recog.base.match_token(C_If,&mut recog.err_handler)?;

					recog.base.set_state(993);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(994);
					recog.expression()?;

					recog.base.set_state(995);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(996);
					recog.statement()?;

					recog.base.set_state(999);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(116,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(997);
							recog.base.match_token(C_Else,&mut recog.err_handler)?;

							/*InvokeRule statement*/
							recog.base.set_state(998);
							recog.statement()?;

							}
						}

						_ => {}
					}
					}
				}

			C_Switch 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1001);
					recog.base.match_token(C_Switch,&mut recog.err_handler)?;

					recog.base.set_state(1002);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1003);
					recog.expression()?;

					recog.base.set_state(1004);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1005);
					recog.statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- iterationStatement ----------------
pub type IterationStatementContextAll<'input> = IterationStatementContext<'input>;


pub type IterationStatementContext<'input> = BaseParserRuleContext<'input,IterationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IterationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for IterationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for IterationStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_iterationStatement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_iterationStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IterationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iterationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iterationStatement }
}
antlr4rust::tid!{IterationStatementContextExt<'a>}

impl<'input> IterationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IterationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IterationStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IterationStatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<IterationStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token While
/// Returns `None` if there is no child corresponding to token While
fn While(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_While, 0)
}
/// Retrieves first TerminalNode corresponding to token LeftParen
/// Returns `None` if there is no child corresponding to token LeftParen
fn LeftParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_LeftParen, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RightParen
/// Returns `None` if there is no child corresponding to token RightParen
fn RightParen(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_RightParen, 0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Do
/// Returns `None` if there is no child corresponding to token Do
fn Do(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Do, 0)
}
/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
/// Retrieves first TerminalNode corresponding to token For
/// Returns `None` if there is no child corresponding to token For
fn For(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_For, 0)
}
fn forCondition(&self) -> Option<Rc<ForConditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IterationStatementContextAttrs<'input> for IterationStatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn iterationStatement(&mut self,)
	-> Result<Rc<IterationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IterationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_iterationStatement);
        let mut _localctx: Rc<IterationStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1029);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			C_While 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(1009);
					recog.base.match_token(C_While,&mut recog.err_handler)?;

					recog.base.set_state(1010);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1011);
					recog.expression()?;

					recog.base.set_state(1012);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1013);
					recog.statement()?;

					}
				}

			C_Do 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(1015);
					recog.base.match_token(C_Do,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1016);
					recog.statement()?;

					recog.base.set_state(1017);
					recog.base.match_token(C_While,&mut recog.err_handler)?;

					recog.base.set_state(1018);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(1019);
					recog.expression()?;

					recog.base.set_state(1020);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					recog.base.set_state(1021);
					recog.base.match_token(C_Semi,&mut recog.err_handler)?;

					}
				}

			C_For 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1023);
					recog.base.match_token(C_For,&mut recog.err_handler)?;

					recog.base.set_state(1024);
					recog.base.match_token(C_LeftParen,&mut recog.err_handler)?;

					/*InvokeRule forCondition*/
					recog.base.set_state(1025);
					recog.forCondition()?;

					recog.base.set_state(1026);
					recog.base.match_token(C_RightParen,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(1027);
					recog.statement()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- forCondition ----------------
pub type ForConditionContextAll<'input> = ForConditionContext<'input>;


pub type ForConditionContext<'input> = BaseParserRuleContext<'input,ForConditionContextExt<'input>>;

#[derive(Clone)]
pub struct ForConditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ForConditionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ForConditionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_forCondition(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_forCondition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ForConditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forCondition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forCondition }
}
antlr4rust::tid!{ForConditionContextExt<'a>}

impl<'input> ForConditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ForConditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForConditionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ForConditionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ForConditionContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token Semi in current rule
fn Semi_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Semi, starting from 0.
/// Returns `None` if number of children corresponding to token Semi is less or equal than `i`.
fn Semi(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, i)
}
fn forDeclaration(&self) -> Option<Rc<ForDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forExpression_all(&self) ->  Vec<Rc<ForExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn forExpression(&self, i: usize) -> Option<Rc<ForExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForConditionContextAttrs<'input> for ForConditionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn forCondition(&mut self,)
	-> Result<Rc<ForConditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForConditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_forCondition);
        let mut _localctx: Rc<ForConditionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1035);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(120,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule forDeclaration*/
					recog.base.set_state(1031);
					recog.forDeclaration()?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(1033);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1032);
						recog.expression()?;

						}
					}

					}
				}

				_ => {}
			}
			recog.base.set_state(1037);
			recog.base.match_token(C_Semi,&mut recog.err_handler)?;

			recog.base.set_state(1039);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
				{
				/*InvokeRule forExpression*/
				recog.base.set_state(1038);
				recog.forExpression()?;

				}
			}

			recog.base.set_state(1041);
			recog.base.match_token(C_Semi,&mut recog.err_handler)?;

			recog.base.set_state(1043);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
				{
				/*InvokeRule forExpression*/
				recog.base.set_state(1042);
				recog.forExpression()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- forDeclaration ----------------
pub type ForDeclarationContextAll<'input> = ForDeclarationContext<'input>;


pub type ForDeclarationContext<'input> = BaseParserRuleContext<'input,ForDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ForDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ForDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ForDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_forDeclaration(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_forDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ForDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forDeclaration }
}
antlr4rust::tid!{ForDeclarationContextExt<'a>}

impl<'input> ForDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ForDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ForDeclarationContextAttrs<'input>: CParserContext<'input> + BorrowMut<ForDeclarationContextExt<'input>>{

fn declarationSpecifiers(&self) -> Option<Rc<DeclarationSpecifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn initDeclaratorList(&self) -> Option<Rc<InitDeclaratorListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForDeclarationContextAttrs<'input> for ForDeclarationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn forDeclaration(&mut self,)
	-> Result<Rc<ForDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_forDeclaration);
        let mut _localctx: Rc<ForDeclarationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule declarationSpecifiers*/
			recog.base.set_state(1045);
			recog.declarationSpecifiers()?;

			recog.base.set_state(1047);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 64000) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & 8454145) != 0) || _la==C_Identifier {
				{
				/*InvokeRule initDeclaratorList*/
				recog.base.set_state(1046);
				recog.initDeclaratorList()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- forExpression ----------------
pub type ForExpressionContextAll<'input> = ForExpressionContext<'input>;


pub type ForExpressionContext<'input> = BaseParserRuleContext<'input,ForExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ForExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ForExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ForExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_forExpression(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_forExpression(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ForExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forExpression }
}
antlr4rust::tid!{ForExpressionContextExt<'a>}

impl<'input> ForExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ForExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForExpressionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ForExpressionContextAttrs<'input>: CParserContext<'input> + BorrowMut<ForExpressionContextExt<'input>>{

fn assignmentExpression_all(&self) ->  Vec<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assignmentExpression(&self, i: usize) -> Option<Rc<AssignmentExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,CParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Comma, i)
}

}

impl<'input> ForExpressionContextAttrs<'input> for ForExpressionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn forExpression(&mut self,)
	-> Result<Rc<ForExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_forExpression);
        let mut _localctx: Rc<ForExpressionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule assignmentExpression*/
			recog.base.set_state(1049);
			recog.assignmentExpression()?;

			recog.base.set_state(1054);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==C_Comma {
				{
				{
				recog.base.set_state(1050);
				recog.base.match_token(C_Comma,&mut recog.err_handler)?;

				/*InvokeRule assignmentExpression*/
				recog.base.set_state(1051);
				recog.assignmentExpression()?;

				}
				}
				recog.base.set_state(1056);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- jumpStatement ----------------
pub type JumpStatementContextAll<'input> = JumpStatementContext<'input>;


pub type JumpStatementContext<'input> = BaseParserRuleContext<'input,JumpStatementContextExt<'input>>;

#[derive(Clone)]
pub struct JumpStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for JumpStatementContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for JumpStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_jumpStatement(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_jumpStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for JumpStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_jumpStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_jumpStatement }
}
antlr4rust::tid!{JumpStatementContextExt<'a>}

impl<'input> JumpStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<JumpStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,JumpStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait JumpStatementContextAttrs<'input>: CParserContext<'input> + BorrowMut<JumpStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}
/// Retrieves first TerminalNode corresponding to token Goto
/// Returns `None` if there is no child corresponding to token Goto
fn Goto(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Goto, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token Continue
/// Returns `None` if there is no child corresponding to token Continue
fn Continue(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Continue, 0)
}
/// Retrieves first TerminalNode corresponding to token Break
/// Returns `None` if there is no child corresponding to token Break
fn Break(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Break, 0)
}
/// Retrieves first TerminalNode corresponding to token Return
/// Returns `None` if there is no child corresponding to token Return
fn Return(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Return, 0)
}
fn unaryExpression(&self) -> Option<Rc<UnaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> JumpStatementContextAttrs<'input> for JumpStatementContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn jumpStatement(&mut self,)
	-> Result<Rc<JumpStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = JumpStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_jumpStatement);
        let mut _localctx: Rc<JumpStatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1067);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(126,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(1057);
					recog.base.match_token(C_Goto,&mut recog.err_handler)?;

					recog.base.set_state(1058);
					recog.base.match_token(C_Identifier,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(1059);
					recog.base.match_token(C_Continue,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(1060);
					recog.base.match_token(C_Break,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					recog.base.set_state(1061);
					recog.base.match_token(C_Return,&mut recog.err_handler)?;

					recog.base.set_state(1063);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) || ((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & 1083393) != 0) || ((((_la - 76)) & !0x3f) == 0 && ((1usize << (_la - 76)) & 12959) != 0) || ((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & 15) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1062);
						recog.expression()?;

						}
					}

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(1065);
					recog.base.match_token(C_Goto,&mut recog.err_handler)?;

					/*InvokeRule unaryExpression*/
					recog.base.set_state(1066);
					recog.unaryExpression()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(1069);
			recog.base.match_token(C_Semi,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- compilationUnit ----------------
pub type CompilationUnitContextAll<'input> = CompilationUnitContext<'input>;


pub type CompilationUnitContext<'input> = BaseParserRuleContext<'input,CompilationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct CompilationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for CompilationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for CompilationUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_compilationUnit(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_compilationUnit(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for CompilationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compilationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compilationUnit }
}
antlr4rust::tid!{CompilationUnitContextExt<'a>}

impl<'input> CompilationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CompilationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompilationUnitContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CompilationUnitContextAttrs<'input>: CParserContext<'input> + BorrowMut<CompilationUnitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_EOF, 0)
}
fn translationUnit(&self) -> Option<Rc<TranslationUnitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompilationUnitContextAttrs<'input> for CompilationUnitContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn compilationUnit(&mut self,)
	-> Result<Rc<CompilationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompilationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_compilationUnit);
        let mut _localctx: Rc<CompilationUnitContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1072);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 1)) & !0x3f) == 0 && ((1usize << (_la - 1)) & 3905519609) != 0) || ((((_la - 36)) & !0x3f) == 0 && ((1usize << (_la - 36)) & 511047391) != 0) || ((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & 1073746049) != 0) {
				{
				/*InvokeRule translationUnit*/
				recog.base.set_state(1071);
				recog.translationUnit()?;

				}
			}

			recog.base.set_state(1074);
			recog.base.match_token(C_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- translationUnit ----------------
pub type TranslationUnitContextAll<'input> = TranslationUnitContext<'input>;


pub type TranslationUnitContext<'input> = BaseParserRuleContext<'input,TranslationUnitContextExt<'input>>;

#[derive(Clone)]
pub struct TranslationUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for TranslationUnitContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for TranslationUnitContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_translationUnit(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_translationUnit(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TranslationUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_translationUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_translationUnit }
}
antlr4rust::tid!{TranslationUnitContextExt<'a>}

impl<'input> TranslationUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TranslationUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TranslationUnitContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TranslationUnitContextAttrs<'input>: CParserContext<'input> + BorrowMut<TranslationUnitContextExt<'input>>{

fn externalDeclaration_all(&self) ->  Vec<Rc<ExternalDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn externalDeclaration(&self, i: usize) -> Option<Rc<ExternalDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TranslationUnitContextAttrs<'input> for TranslationUnitContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn translationUnit(&mut self,)
	-> Result<Rc<TranslationUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TranslationUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_translationUnit);
        let mut _localctx: Rc<TranslationUnitContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1077); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule externalDeclaration*/
				recog.base.set_state(1076);
				recog.externalDeclaration()?;

				}
				}
				recog.base.set_state(1079); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(((((_la - 1)) & !0x3f) == 0 && ((1usize << (_la - 1)) & 3905519609) != 0) || ((((_la - 36)) & !0x3f) == 0 && ((1usize << (_la - 36)) & 511047391) != 0) || ((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & 1073746049) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- externalDeclaration ----------------
pub type ExternalDeclarationContextAll<'input> = ExternalDeclarationContext<'input>;


pub type ExternalDeclarationContext<'input> = BaseParserRuleContext<'input,ExternalDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ExternalDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for ExternalDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for ExternalDeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_externalDeclaration(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_externalDeclaration(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ExternalDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_externalDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_externalDeclaration }
}
antlr4rust::tid!{ExternalDeclarationContextExt<'a>}

impl<'input> ExternalDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExternalDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExternalDeclarationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExternalDeclarationContextAttrs<'input>: CParserContext<'input> + BorrowMut<ExternalDeclarationContextExt<'input>>{

fn functionDefinition(&self) -> Option<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declaration(&self) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Semi
/// Returns `None` if there is no child corresponding to token Semi
fn Semi(&self) -> Option<Rc<TerminalNode<'input,CParserContextType>>> where Self:Sized{
	self.get_token(C_Semi, 0)
}

}

impl<'input> ExternalDeclarationContextAttrs<'input> for ExternalDeclarationContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn externalDeclaration(&mut self,)
	-> Result<Rc<ExternalDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExternalDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_externalDeclaration);
        let mut _localctx: Rc<ExternalDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(1084);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(129,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(1081);
					recog.functionDefinition()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule declaration*/
					recog.base.set_state(1082);
					recog.declaration()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(1083);
					recog.base.match_token(C_Semi,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- functionDefinition ----------------
pub type FunctionDefinitionContextAll<'input> = FunctionDefinitionContext<'input>;


pub type FunctionDefinitionContext<'input> = BaseParserRuleContext<'input,FunctionDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for FunctionDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for FunctionDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_functionDefinition(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_functionDefinition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FunctionDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDefinition }
}
antlr4rust::tid!{FunctionDefinitionContextExt<'a>}

impl<'input> FunctionDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FunctionDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDefinitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDefinitionContextAttrs<'input>: CParserContext<'input> + BorrowMut<FunctionDefinitionContextExt<'input>>{

fn declarator(&self) -> Option<Rc<DeclaratorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn compoundStatement(&self) -> Option<Rc<CompoundStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declarationSpecifiers(&self) -> Option<Rc<DeclarationSpecifiersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declarationList(&self) -> Option<Rc<DeclarationListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionDefinitionContextAttrs<'input> for FunctionDefinitionContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn functionDefinition(&mut self,)
	-> Result<Rc<FunctionDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_functionDefinition);
        let mut _localctx: Rc<FunctionDefinitionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1087);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(130,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule declarationSpecifiers*/
					recog.base.set_state(1086);
					recog.declarationSpecifiers()?;

					}
				}

				_ => {}
			}
			/*InvokeRule declarator*/
			recog.base.set_state(1089);
			recog.declarator()?;

			recog.base.set_state(1091);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 3516008434) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 3881790961) != 0) || _la==C_Identifier {
				{
				/*InvokeRule declarationList*/
				recog.base.set_state(1090);
				recog.declarationList()?;

				}
			}

			/*InvokeRule compoundStatement*/
			recog.base.set_state(1093);
			recog.compoundStatement()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declarationList ----------------
pub type DeclarationListContextAll<'input> = DeclarationListContext<'input>;


pub type DeclarationListContext<'input> = BaseParserRuleContext<'input,DeclarationListContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> CParserContext<'input> for DeclarationListContext<'input>{}

impl<'input,'a> Listenable<dyn CListener<'input> + 'a> for DeclarationListContext<'input>{
		fn enter(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declarationList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn CListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declarationList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = CParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declarationList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declarationList }
}
antlr4rust::tid!{DeclarationListContextExt<'a>}

impl<'input> DeclarationListContextExt<'input>{
	fn new(parent: Option<Rc<dyn CParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DeclarationListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationListContextAttrs<'input>: CParserContext<'input> + BorrowMut<DeclarationListContextExt<'input>>{

fn declaration_all(&self) ->  Vec<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declaration(&self, i: usize) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DeclarationListContextAttrs<'input> for DeclarationListContext<'input>{}

impl<'input, I> CParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declarationList(&mut self,)
	-> Result<Rc<DeclarationListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_declarationList);
        let mut _localctx: Rc<DeclarationListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(1096); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule declaration*/
				recog.base.set_state(1095);
				recog.declaration()?;

				}
				}
				recog.base.set_state(1098); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 3516008434) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 3881790961) != 0) || _la==C_Identifier) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 120, 1101, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
		7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 
		7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 
		7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 
		7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 
		7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 
		7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 
		7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 
		7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2, 65, 
		7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 
		7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 
		7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 2, 79, 7, 79, 2, 80, 
		7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 84, 7, 84, 2, 85, 
		7, 85, 2, 86, 7, 86, 1, 0, 1, 0, 1, 0, 4, 0, 178, 8, 0, 11, 0, 12, 0, 
		179, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 3, 0, 188, 8, 0, 1, 0, 1, 0, 
		1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 
		1, 0, 1, 0, 1, 0, 1, 0, 3, 0, 208, 8, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
		1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 5, 2, 220, 8, 2, 10, 2, 12, 2, 223, 9, 2, 
		1, 3, 1, 3, 3, 3, 227, 8, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 3, 4, 234, 
		8, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3, 4, 242, 8, 4, 1, 4, 1, 4, 
		3, 4, 246, 8, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 3, 4, 254, 8, 4, 
		1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 5, 4, 261, 8, 4, 10, 4, 12, 4, 264, 9, 4, 
		1, 5, 1, 5, 1, 5, 5, 5, 269, 8, 5, 10, 5, 12, 5, 272, 9, 5, 1, 6, 5, 6, 
		275, 8, 6, 10, 6, 12, 6, 278, 9, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 
		1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 3, 6, 291, 8, 6, 1, 7, 1, 7, 1, 8, 3, 8, 
		296, 8, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 3, 8, 305, 8, 8, 
		1, 9, 1, 9, 1, 9, 5, 9, 310, 8, 9, 10, 9, 12, 9, 313, 9, 9, 1, 10, 1, 
		10, 1, 10, 5, 10, 318, 8, 10, 10, 10, 12, 10, 321, 9, 10, 1, 11, 1, 11, 
		1, 11, 5, 11, 326, 8, 11, 10, 11, 12, 11, 329, 9, 11, 1, 12, 1, 12, 1, 
		12, 5, 12, 334, 8, 12, 10, 12, 12, 12, 337, 9, 12, 1, 13, 1, 13, 1, 13, 
		5, 13, 342, 8, 13, 10, 13, 12, 13, 345, 9, 13, 1, 14, 1, 14, 1, 14, 5, 
		14, 350, 8, 14, 10, 14, 12, 14, 353, 9, 14, 1, 15, 1, 15, 1, 15, 5, 15, 
		358, 8, 15, 10, 15, 12, 15, 361, 9, 15, 1, 16, 1, 16, 1, 16, 5, 16, 366, 
		8, 16, 10, 16, 12, 16, 369, 9, 16, 1, 17, 1, 17, 1, 17, 5, 17, 374, 8, 
		17, 10, 17, 12, 17, 377, 9, 17, 1, 18, 1, 18, 1, 18, 5, 18, 382, 8, 18, 
		10, 18, 12, 18, 385, 9, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 
		3, 19, 393, 8, 19, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 3, 20, 401, 
		8, 20, 1, 21, 1, 21, 1, 22, 1, 22, 1, 22, 5, 22, 408, 8, 22, 10, 22, 12, 
		22, 411, 9, 22, 1, 23, 1, 23, 1, 24, 1, 24, 3, 24, 417, 8, 24, 1, 24, 
		1, 24, 1, 24, 3, 24, 422, 8, 24, 1, 25, 4, 25, 425, 8, 25, 11, 25, 12, 
		25, 426, 1, 26, 4, 26, 430, 8, 26, 11, 26, 12, 26, 431, 1, 27, 1, 27, 
		1, 27, 1, 27, 1, 27, 3, 27, 439, 8, 27, 1, 28, 1, 28, 1, 28, 5, 28, 444, 
		8, 28, 10, 28, 12, 28, 447, 9, 28, 1, 29, 1, 29, 1, 29, 3, 29, 452, 8, 
		29, 1, 30, 1, 30, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 
		31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 
		31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 3, 
		31, 483, 8, 31, 1, 32, 1, 32, 3, 32, 487, 8, 32, 1, 32, 1, 32, 1, 32, 
		1, 32, 1, 32, 1, 32, 1, 32, 3, 32, 496, 8, 32, 1, 33, 1, 33, 1, 34, 4, 
		34, 501, 8, 34, 11, 34, 12, 34, 502, 1, 35, 1, 35, 1, 35, 1, 35, 1, 35, 
		1, 35, 1, 35, 1, 35, 3, 35, 513, 8, 35, 1, 36, 1, 36, 3, 36, 517, 8, 36, 
		1, 36, 3, 36, 520, 8, 36, 1, 37, 1, 37, 1, 37, 5, 37, 525, 8, 37, 10, 
		37, 12, 37, 528, 9, 37, 1, 38, 1, 38, 3, 38, 532, 8, 38, 1, 38, 1, 38, 
		3, 38, 536, 8, 38, 1, 39, 1, 39, 3, 39, 540, 8, 39, 1, 39, 1, 39, 1, 39, 
		3, 39, 545, 8, 39, 1, 39, 1, 39, 1, 39, 1, 39, 3, 39, 551, 8, 39, 1, 40, 
		1, 40, 1, 40, 5, 40, 556, 8, 40, 10, 40, 12, 40, 559, 9, 40, 1, 41, 1, 
		41, 1, 41, 3, 41, 564, 8, 41, 1, 42, 1, 42, 1, 43, 1, 43, 1, 43, 1, 43, 
		1, 43, 1, 44, 1, 44, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 
		1, 45, 1, 45, 3, 45, 584, 8, 45, 1, 46, 1, 46, 1, 46, 1, 46, 3, 46, 590, 
		8, 46, 1, 46, 1, 46, 1, 47, 3, 47, 595, 8, 47, 1, 47, 1, 47, 5, 47, 599, 
		8, 47, 10, 47, 12, 47, 602, 9, 47, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 
		1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 
		1, 48, 1, 48, 3, 48, 621, 8, 48, 1, 48, 1, 48, 1, 48, 3, 48, 626, 8, 48, 
		1, 48, 3, 48, 629, 8, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 3, 48, 636, 
		8, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 
		1, 48, 1, 48, 1, 48, 1, 48, 3, 48, 651, 8, 48, 1, 48, 1, 48, 1, 48, 1, 
		48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 1, 48, 3, 48, 663, 8, 48, 1, 48, 
		5, 48, 666, 8, 48, 10, 48, 12, 48, 669, 9, 48, 1, 49, 1, 49, 1, 50, 1, 
		50, 1, 50, 4, 50, 676, 8, 50, 11, 50, 12, 50, 677, 1, 50, 1, 50, 3, 50, 
		682, 8, 50, 1, 51, 1, 51, 1, 51, 1, 51, 1, 51, 1, 51, 1, 51, 1, 52, 3, 
		52, 692, 8, 52, 1, 52, 1, 52, 3, 52, 696, 8, 52, 5, 52, 698, 8, 52, 10, 
		52, 12, 52, 701, 9, 52, 1, 53, 1, 53, 1, 53, 3, 53, 706, 8, 53, 1, 53, 
		3, 53, 709, 8, 53, 1, 54, 1, 54, 3, 54, 713, 8, 54, 4, 54, 715, 8, 54, 
		11, 54, 12, 54, 716, 1, 55, 4, 55, 720, 8, 55, 11, 55, 12, 55, 721, 1, 
		56, 1, 56, 1, 56, 3, 56, 727, 8, 56, 1, 57, 1, 57, 1, 57, 5, 57, 732, 
		8, 57, 10, 57, 12, 57, 735, 9, 57, 1, 58, 1, 58, 1, 58, 1, 58, 1, 58, 
		3, 58, 742, 8, 58, 3, 58, 744, 8, 58, 1, 59, 1, 59, 1, 59, 5, 59, 749, 
		8, 59, 10, 59, 12, 59, 752, 9, 59, 1, 60, 1, 60, 3, 60, 756, 8, 60, 1, 
		61, 1, 61, 3, 61, 760, 8, 61, 1, 61, 1, 61, 5, 61, 764, 8, 61, 10, 61, 
		12, 61, 767, 9, 61, 3, 61, 769, 8, 61, 1, 62, 1, 62, 1, 62, 1, 62, 1, 
		62, 5, 62, 776, 8, 62, 10, 62, 12, 62, 779, 9, 62, 1, 62, 1, 62, 3, 62, 
		783, 8, 62, 1, 62, 3, 62, 786, 8, 62, 1, 62, 1, 62, 1, 62, 1, 62, 3, 62, 
		792, 8, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 
		62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 3, 62, 808, 8, 62, 1, 62, 1, 62, 
		5, 62, 812, 8, 62, 10, 62, 12, 62, 815, 9, 62, 3, 62, 817, 8, 62, 1, 62, 
		1, 62, 1, 62, 3, 62, 822, 8, 62, 1, 62, 3, 62, 825, 8, 62, 1, 62, 1, 62, 
		1, 62, 1, 62, 1, 62, 3, 62, 832, 8, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 
		62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 1, 
		62, 1, 62, 1, 62, 3, 62, 851, 8, 62, 1, 62, 1, 62, 5, 62, 855, 8, 62, 
		10, 62, 12, 62, 858, 9, 62, 5, 62, 860, 8, 62, 10, 62, 12, 62, 863, 9, 
		62, 1, 63, 1, 63, 1, 64, 1, 64, 1, 64, 1, 64, 3, 64, 871, 8, 64, 1, 64, 
		1, 64, 3, 64, 875, 8, 64, 1, 65, 3, 65, 878, 8, 65, 1, 65, 1, 65, 1, 65, 
		3, 65, 883, 8, 65, 1, 65, 5, 65, 886, 8, 65, 10, 65, 12, 65, 889, 9, 65, 
		1, 66, 1, 66, 1, 66, 1, 67, 4, 67, 895, 8, 67, 11, 67, 12, 67, 896, 1, 
		68, 1, 68, 1, 68, 1, 68, 1, 68, 1, 68, 3, 68, 905, 8, 68, 1, 69, 1, 69, 
		1, 69, 1, 69, 1, 69, 4, 69, 912, 8, 69, 11, 69, 12, 69, 913, 1, 69, 1, 
		69, 1, 69, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 
		70, 1, 70, 1, 70, 1, 70, 5, 70, 931, 8, 70, 10, 70, 12, 70, 934, 9, 70, 
		3, 70, 936, 8, 70, 1, 70, 1, 70, 1, 70, 1, 70, 5, 70, 942, 8, 70, 10, 
		70, 12, 70, 945, 9, 70, 3, 70, 947, 8, 70, 5, 70, 949, 8, 70, 10, 70, 
		12, 70, 952, 9, 70, 1, 70, 1, 70, 3, 70, 956, 8, 70, 1, 71, 1, 71, 1, 
		71, 3, 71, 961, 8, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 71, 
		1, 71, 3, 71, 971, 8, 71, 1, 72, 1, 72, 3, 72, 975, 8, 72, 1, 72, 1, 72, 
		1, 73, 4, 73, 980, 8, 73, 11, 73, 12, 73, 981, 1, 74, 1, 74, 3, 74, 986, 
		8, 74, 1, 75, 3, 75, 989, 8, 75, 1, 75, 1, 75, 1, 76, 1, 76, 1, 76, 1, 
		76, 1, 76, 1, 76, 1, 76, 3, 76, 1000, 8, 76, 1, 76, 1, 76, 1, 76, 1, 76, 
		1, 76, 1, 76, 3, 76, 1008, 8, 76, 1, 77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 
		77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 77, 1, 
		77, 1, 77, 1, 77, 1, 77, 1, 77, 3, 77, 1030, 8, 77, 1, 78, 1, 78, 3, 78, 
		1034, 8, 78, 3, 78, 1036, 8, 78, 1, 78, 1, 78, 3, 78, 1040, 8, 78, 1, 
		78, 1, 78, 3, 78, 1044, 8, 78, 1, 79, 1, 79, 3, 79, 1048, 8, 79, 1, 80, 
		1, 80, 1, 80, 5, 80, 1053, 8, 80, 10, 80, 12, 80, 1056, 9, 80, 1, 81, 
		1, 81, 1, 81, 1, 81, 1, 81, 1, 81, 3, 81, 1064, 8, 81, 1, 81, 1, 81, 3, 
		81, 1068, 8, 81, 1, 81, 1, 81, 1, 82, 3, 82, 1073, 8, 82, 1, 82, 1, 82, 
		1, 83, 4, 83, 1078, 8, 83, 11, 83, 12, 83, 1079, 1, 84, 1, 84, 1, 84, 
		3, 84, 1085, 8, 84, 1, 85, 3, 85, 1088, 8, 85, 1, 85, 1, 85, 3, 85, 1092, 
		8, 85, 1, 85, 1, 85, 1, 86, 4, 86, 1097, 8, 86, 11, 86, 12, 86, 1098, 
		1, 86, 0, 2, 96, 124, 87, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 
		26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 
		62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 
		98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 
		128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 
		156, 158, 160, 162, 164, 166, 168, 170, 172, 0, 19, 1, 0, 107, 108, 3, 
		0, 44, 44, 77, 77, 79, 79, 2, 0, 44, 44, 55, 55, 5, 0, 76, 76, 78, 78, 
		80, 80, 83, 83, 88, 89, 1, 0, 80, 82, 2, 0, 76, 76, 78, 78, 1, 0, 74, 
		75, 1, 0, 70, 73, 1, 0, 105, 106, 1, 0, 94, 104, 6, 0, 20, 20, 31, 31, 
		39, 39, 45, 45, 48, 48, 63, 63, 1, 0, 4, 6, 2, 0, 46, 46, 49, 49, 4, 0, 
		24, 24, 40, 40, 52, 52, 56, 56, 2, 0, 9, 9, 11, 15, 2, 0, 64, 65, 93, 
		93, 2, 0, 80, 80, 87, 87, 2, 0, 16, 16, 18, 18, 2, 0, 19, 19, 52, 52, 
		1210, 0, 207, 1, 0, 0, 0, 2, 209, 1, 0, 0, 0, 4, 216, 1, 0, 0, 0, 6, 226, 
		1, 0, 0, 0, 8, 245, 1, 0, 0, 0, 10, 265, 1, 0, 0, 0, 12, 276, 1, 0, 0, 
		0, 14, 292, 1, 0, 0, 0, 16, 304, 1, 0, 0, 0, 18, 306, 1, 0, 0, 0, 20, 
		314, 1, 0, 0, 0, 22, 322, 1, 0, 0, 0, 24, 330, 1, 0, 0, 0, 26, 338, 1, 
		0, 0, 0, 28, 346, 1, 0, 0, 0, 30, 354, 1, 0, 0, 0, 32, 362, 1, 0, 0, 0, 
		34, 370, 1, 0, 0, 0, 36, 378, 1, 0, 0, 0, 38, 386, 1, 0, 0, 0, 40, 400, 
		1, 0, 0, 0, 42, 402, 1, 0, 0, 0, 44, 404, 1, 0, 0, 0, 46, 412, 1, 0, 0, 
		0, 48, 421, 1, 0, 0, 0, 50, 424, 1, 0, 0, 0, 52, 429, 1, 0, 0, 0, 54, 
		438, 1, 0, 0, 0, 56, 440, 1, 0, 0, 0, 58, 448, 1, 0, 0, 0, 60, 453, 1, 
		0, 0, 0, 62, 482, 1, 0, 0, 0, 64, 495, 1, 0, 0, 0, 66, 497, 1, 0, 0, 0, 
		68, 500, 1, 0, 0, 0, 70, 512, 1, 0, 0, 0, 72, 516, 1, 0, 0, 0, 74, 521, 
		1, 0, 0, 0, 76, 535, 1, 0, 0, 0, 78, 550, 1, 0, 0, 0, 80, 552, 1, 0, 0, 
		0, 82, 560, 1, 0, 0, 0, 84, 565, 1, 0, 0, 0, 86, 567, 1, 0, 0, 0, 88, 
		572, 1, 0, 0, 0, 90, 583, 1, 0, 0, 0, 92, 585, 1, 0, 0, 0, 94, 594, 1, 
		0, 0, 0, 96, 620, 1, 0, 0, 0, 98, 670, 1, 0, 0, 0, 100, 681, 1, 0, 0, 
		0, 102, 683, 1, 0, 0, 0, 104, 691, 1, 0, 0, 0, 106, 702, 1, 0, 0, 0, 108, 
		714, 1, 0, 0, 0, 110, 719, 1, 0, 0, 0, 112, 723, 1, 0, 0, 0, 114, 728, 
		1, 0, 0, 0, 116, 743, 1, 0, 0, 0, 118, 745, 1, 0, 0, 0, 120, 753, 1, 0, 
		0, 0, 122, 768, 1, 0, 0, 0, 124, 816, 1, 0, 0, 0, 126, 864, 1, 0, 0, 0, 
		128, 874, 1, 0, 0, 0, 130, 877, 1, 0, 0, 0, 132, 890, 1, 0, 0, 0, 134, 
		894, 1, 0, 0, 0, 136, 904, 1, 0, 0, 0, 138, 906, 1, 0, 0, 0, 140, 955, 
		1, 0, 0, 0, 142, 970, 1, 0, 0, 0, 144, 972, 1, 0, 0, 0, 146, 979, 1, 0, 
		0, 0, 148, 985, 1, 0, 0, 0, 150, 988, 1, 0, 0, 0, 152, 1007, 1, 0, 0, 
		0, 154, 1029, 1, 0, 0, 0, 156, 1035, 1, 0, 0, 0, 158, 1045, 1, 0, 0, 0, 
		160, 1049, 1, 0, 0, 0, 162, 1067, 1, 0, 0, 0, 164, 1072, 1, 0, 0, 0, 166, 
		1077, 1, 0, 0, 0, 168, 1084, 1, 0, 0, 0, 170, 1087, 1, 0, 0, 0, 172, 1096, 
		1, 0, 0, 0, 174, 208, 5, 110, 0, 0, 175, 208, 5, 111, 0, 0, 176, 178, 
		5, 113, 0, 0, 177, 176, 1, 0, 0, 0, 178, 179, 1, 0, 0, 0, 179, 177, 1, 
		0, 0, 0, 179, 180, 1, 0, 0, 0, 180, 208, 1, 0, 0, 0, 181, 182, 5, 64, 
		0, 0, 182, 183, 3, 44, 22, 0, 183, 184, 5, 65, 0, 0, 184, 208, 1, 0, 0, 
		0, 185, 208, 3, 2, 1, 0, 186, 188, 5, 1, 0, 0, 187, 186, 1, 0, 0, 0, 187, 
		188, 1, 0, 0, 0, 188, 189, 1, 0, 0, 0, 189, 190, 5, 64, 0, 0, 190, 191, 
		3, 144, 72, 0, 191, 192, 5, 65, 0, 0, 192, 208, 1, 0, 0, 0, 193, 194, 
		5, 2, 0, 0, 194, 195, 5, 64, 0, 0, 195, 196, 3, 12, 6, 0, 196, 197, 5, 
		93, 0, 0, 197, 198, 3, 120, 60, 0, 198, 199, 5, 65, 0, 0, 199, 208, 1, 
		0, 0, 0, 200, 201, 5, 3, 0, 0, 201, 202, 5, 64, 0, 0, 202, 203, 3, 120, 
		60, 0, 203, 204, 5, 93, 0, 0, 204, 205, 3, 12, 6, 0, 205, 206, 5, 65, 
		0, 0, 206, 208, 1, 0, 0, 0, 207, 174, 1, 0, 0, 0, 207, 175, 1, 0, 0, 0, 
		207, 177, 1, 0, 0, 0, 207, 181, 1, 0, 0, 0, 207, 185, 1, 0, 0, 0, 207, 
		187, 1, 0, 0, 0, 207, 193, 1, 0, 0, 0, 207, 200, 1, 0, 0, 0, 208, 1, 1, 
		0, 0, 0, 209, 210, 5, 59, 0, 0, 210, 211, 5, 64, 0, 0, 211, 212, 3, 40, 
		20, 0, 212, 213, 5, 93, 0, 0, 213, 214, 3, 4, 2, 0, 214, 215, 5, 65, 0, 
		0, 215, 3, 1, 0, 0, 0, 216, 221, 3, 6, 3, 0, 217, 218, 5, 93, 0, 0, 218, 
		220, 3, 6, 3, 0, 219, 217, 1, 0, 0, 0, 220, 223, 1, 0, 0, 0, 221, 219, 
		1, 0, 0, 0, 221, 222, 1, 0, 0, 0, 222, 5, 1, 0, 0, 0, 223, 221, 1, 0, 
		0, 0, 224, 227, 3, 120, 60, 0, 225, 227, 5, 26, 0, 0, 226, 224, 1, 0, 
		0, 0, 226, 225, 1, 0, 0, 0, 227, 228, 1, 0, 0, 0, 228, 229, 5, 91, 0, 
		0, 229, 230, 3, 40, 20, 0, 230, 7, 1, 0, 0, 0, 231, 246, 3, 0, 0, 0, 232, 
		234, 5, 1, 0, 0, 233, 232, 1, 0, 0, 0, 233, 234, 1, 0, 0, 0, 234, 235, 
		1, 0, 0, 0, 235, 236, 5, 64, 0, 0, 236, 237, 3, 120, 60, 0, 237, 238, 
		5, 65, 0, 0, 238, 239, 5, 68, 0, 0, 239, 241, 3, 130, 65, 0, 240, 242, 
		5, 93, 0, 0, 241, 240, 1, 0, 0, 0, 241, 242, 1, 0, 0, 0, 242, 243, 1, 
		0, 0, 0, 243, 244, 5, 69, 0, 0, 244, 246, 1, 0, 0, 0, 245, 231, 1, 0, 
		0, 0, 245, 233, 1, 0, 0, 0, 246, 262, 1, 0, 0, 0, 247, 248, 5, 66, 0, 
		0, 248, 249, 3, 44, 22, 0, 249, 250, 5, 67, 0, 0, 250, 261, 1, 0, 0, 0, 
		251, 253, 5, 64, 0, 0, 252, 254, 3, 10, 5, 0, 253, 252, 1, 0, 0, 0, 253, 
		254, 1, 0, 0, 0, 254, 255, 1, 0, 0, 0, 255, 261, 5, 65, 0, 0, 256, 257, 
		7, 0, 0, 0, 257, 261, 5, 110, 0, 0, 258, 261, 5, 77, 0, 0, 259, 261, 5, 
		79, 0, 0, 260, 247, 1, 0, 0, 0, 260, 251, 1, 0, 0, 0, 260, 256, 1, 0, 
		0, 0, 260, 258, 1, 0, 0, 0, 260, 259, 1, 0, 0, 0, 261, 264, 1, 0, 0, 0, 
		262, 260, 1, 0, 0, 0, 262, 263, 1, 0, 0, 0, 263, 9, 1, 0, 0, 0, 264, 262, 
		1, 0, 0, 0, 265, 270, 3, 40, 20, 0, 266, 267, 5, 93, 0, 0, 267, 269, 3, 
		40, 20, 0, 268, 266, 1, 0, 0, 0, 269, 272, 1, 0, 0, 0, 270, 268, 1, 0, 
		0, 0, 270, 271, 1, 0, 0, 0, 271, 11, 1, 0, 0, 0, 272, 270, 1, 0, 0, 0, 
		273, 275, 7, 1, 0, 0, 274, 273, 1, 0, 0, 0, 275, 278, 1, 0, 0, 0, 276, 
		274, 1, 0, 0, 0, 276, 277, 1, 0, 0, 0, 277, 290, 1, 0, 0, 0, 278, 276, 
		1, 0, 0, 0, 279, 291, 3, 8, 4, 0, 280, 281, 3, 14, 7, 0, 281, 282, 3, 
		16, 8, 0, 282, 291, 1, 0, 0, 0, 283, 284, 7, 2, 0, 0, 284, 285, 5, 64, 
		0, 0, 285, 286, 3, 120, 60, 0, 286, 287, 5, 65, 0, 0, 287, 291, 1, 0, 
		0, 0, 288, 289, 5, 85, 0, 0, 289, 291, 5, 110, 0, 0, 290, 279, 1, 0, 0, 
		0, 290, 280, 1, 0, 0, 0, 290, 283, 1, 0, 0, 0, 290, 288, 1, 0, 0, 0, 291, 
		13, 1, 0, 0, 0, 292, 293, 7, 3, 0, 0, 293, 15, 1, 0, 0, 0, 294, 296, 5, 
		1, 0, 0, 295, 294, 1, 0, 0, 0, 295, 296, 1, 0, 0, 0, 296, 297, 1, 0, 0, 
		0, 297, 298, 5, 64, 0, 0, 298, 299, 3, 120, 60, 0, 299, 300, 5, 65, 0, 
		0, 300, 301, 3, 16, 8, 0, 301, 305, 1, 0, 0, 0, 302, 305, 3, 12, 6, 0, 
		303, 305, 5, 112, 0, 0, 304, 295, 1, 0, 0, 0, 304, 302, 1, 0, 0, 0, 304, 
		303, 1, 0, 0, 0, 305, 17, 1, 0, 0, 0, 306, 311, 3, 16, 8, 0, 307, 308, 
		7, 4, 0, 0, 308, 310, 3, 16, 8, 0, 309, 307, 1, 0, 0, 0, 310, 313, 1, 
		0, 0, 0, 311, 309, 1, 0, 0, 0, 311, 312, 1, 0, 0, 0, 312, 19, 1, 0, 0, 
		0, 313, 311, 1, 0, 0, 0, 314, 319, 3, 18, 9, 0, 315, 316, 7, 5, 0, 0, 
		316, 318, 3, 18, 9, 0, 317, 315, 1, 0, 0, 0, 318, 321, 1, 0, 0, 0, 319, 
		317, 1, 0, 0, 0, 319, 320, 1, 0, 0, 0, 320, 21, 1, 0, 0, 0, 321, 319, 
		1, 0, 0, 0, 322, 327, 3, 20, 10, 0, 323, 324, 7, 6, 0, 0, 324, 326, 3, 
		20, 10, 0, 325, 323, 1, 0, 0, 0, 326, 329, 1, 0, 0, 0, 327, 325, 1, 0, 
		0, 0, 327, 328, 1, 0, 0, 0, 328, 23, 1, 0, 0, 0, 329, 327, 1, 0, 0, 0, 
		330, 335, 3, 22, 11, 0, 331, 332, 7, 7, 0, 0, 332, 334, 3, 22, 11, 0, 
		333, 331, 1, 0, 0, 0, 334, 337, 1, 0, 0, 0, 335, 333, 1, 0, 0, 0, 335, 
		336, 1, 0, 0, 0, 336, 25, 1, 0, 0, 0, 337, 335, 1, 0, 0, 0, 338, 343, 
		3, 24, 12, 0, 339, 340, 7, 8, 0, 0, 340, 342, 3, 24, 12, 0, 341, 339, 
		1, 0, 0, 0, 342, 345, 1, 0, 0, 0, 343, 341, 1, 0, 0, 0, 343, 344, 1, 0, 
		0, 0, 344, 27, 1, 0, 0, 0, 345, 343, 1, 0, 0, 0, 346, 351, 3, 26, 13, 
		0, 347, 348, 5, 83, 0, 0, 348, 350, 3, 26, 13, 0, 349, 347, 1, 0, 0, 0, 
		350, 353, 1, 0, 0, 0, 351, 349, 1, 0, 0, 0, 351, 352, 1, 0, 0, 0, 352, 
		29, 1, 0, 0, 0, 353, 351, 1, 0, 0, 0, 354, 359, 3, 28, 14, 0, 355, 356, 
		5, 87, 0, 0, 356, 358, 3, 28, 14, 0, 357, 355, 1, 0, 0, 0, 358, 361, 1, 
		0, 0, 0, 359, 357, 1, 0, 0, 0, 359, 360, 1, 0, 0, 0, 360, 31, 1, 0, 0, 
		0, 361, 359, 1, 0, 0, 0, 362, 367, 3, 30, 15, 0, 363, 364, 5, 84, 0, 0, 
		364, 366, 3, 30, 15, 0, 365, 363, 1, 0, 0, 0, 366, 369, 1, 0, 0, 0, 367, 
		365, 1, 0, 0, 0, 367, 368, 1, 0, 0, 0, 368, 33, 1, 0, 0, 0, 369, 367, 
		1, 0, 0, 0, 370, 375, 3, 32, 16, 0, 371, 372, 5, 85, 0, 0, 372, 374, 3, 
		32, 16, 0, 373, 371, 1, 0, 0, 0, 374, 377, 1, 0, 0, 0, 375, 373, 1, 0, 
		0, 0, 375, 376, 1, 0, 0, 0, 376, 35, 1, 0, 0, 0, 377, 375, 1, 0, 0, 0, 
		378, 383, 3, 34, 17, 0, 379, 380, 5, 86, 0, 0, 380, 382, 3, 34, 17, 0, 
		381, 379, 1, 0, 0, 0, 382, 385, 1, 0, 0, 0, 383, 381, 1, 0, 0, 0, 383, 
		384, 1, 0, 0, 0, 384, 37, 1, 0, 0, 0, 385, 383, 1, 0, 0, 0, 386, 392, 
		3, 36, 18, 0, 387, 388, 5, 90, 0, 0, 388, 389, 3, 44, 22, 0, 389, 390, 
		5, 91, 0, 0, 390, 391, 3, 38, 19, 0, 391, 393, 1, 0, 0, 0, 392, 387, 1, 
		0, 0, 0, 392, 393, 1, 0, 0, 0, 393, 39, 1, 0, 0, 0, 394, 401, 3, 38, 19, 
		0, 395, 396, 3, 12, 6, 0, 396, 397, 3, 42, 21, 0, 397, 398, 3, 40, 20, 
		0, 398, 401, 1, 0, 0, 0, 399, 401, 5, 112, 0, 0, 400, 394, 1, 0, 0, 0, 
		400, 395, 1, 0, 0, 0, 400, 399, 1, 0, 0, 0, 401, 41, 1, 0, 0, 0, 402, 
		403, 7, 9, 0, 0, 403, 43, 1, 0, 0, 0, 404, 409, 3, 40, 20, 0, 405, 406, 
		5, 93, 0, 0, 406, 408, 3, 40, 20, 0, 407, 405, 1, 0, 0, 0, 408, 411, 1, 
		0, 0, 0, 409, 407, 1, 0, 0, 0, 409, 410, 1, 0, 0, 0, 410, 45, 1, 0, 0, 
		0, 411, 409, 1, 0, 0, 0, 412, 413, 3, 38, 19, 0, 413, 47, 1, 0, 0, 0, 
		414, 416, 3, 50, 25, 0, 415, 417, 3, 56, 28, 0, 416, 415, 1, 0, 0, 0, 
		416, 417, 1, 0, 0, 0, 417, 418, 1, 0, 0, 0, 418, 419, 5, 92, 0, 0, 419, 
		422, 1, 0, 0, 0, 420, 422, 3, 138, 69, 0, 421, 414, 1, 0, 0, 0, 421, 420, 
		1, 0, 0, 0, 422, 49, 1, 0, 0, 0, 423, 425, 3, 54, 27, 0, 424, 423, 1, 
		0, 0, 0, 425, 426, 1, 0, 0, 0, 426, 424, 1, 0, 0, 0, 426, 427, 1, 0, 0, 
		0, 427, 51, 1, 0, 0, 0, 428, 430, 3, 54, 27, 0, 429, 428, 1, 0, 0, 0, 
		430, 431, 1, 0, 0, 0, 431, 429, 1, 0, 0, 0, 431, 432, 1, 0, 0, 0, 432, 
		53, 1, 0, 0, 0, 433, 439, 3, 60, 30, 0, 434, 439, 3, 62, 31, 0, 435, 439, 
		3, 88, 44, 0, 436, 439, 3, 90, 45, 0, 437, 439, 3, 92, 46, 0, 438, 433, 
		1, 0, 0, 0, 438, 434, 1, 0, 0, 0, 438, 435, 1, 0, 0, 0, 438, 436, 1, 0, 
		0, 0, 438, 437, 1, 0, 0, 0, 439, 55, 1, 0, 0, 0, 440, 445, 3, 58, 29, 
		0, 441, 442, 5, 93, 0, 0, 442, 444, 3, 58, 29, 0, 443, 441, 1, 0, 0, 0, 
		444, 447, 1, 0, 0, 0, 445, 443, 1, 0, 0, 0, 445, 446, 1, 0, 0, 0, 446, 
		57, 1, 0, 0, 0, 447, 445, 1, 0, 0, 0, 448, 451, 3, 94, 47, 0, 449, 450, 
		5, 94, 0, 0, 450, 452, 3, 128, 64, 0, 451, 449, 1, 0, 0, 0, 451, 452, 
		1, 0, 0, 0, 452, 59, 1, 0, 0, 0, 453, 454, 7, 10, 0, 0, 454, 61, 1, 0, 
		0, 0, 455, 483, 5, 51, 0, 0, 456, 483, 5, 23, 0, 0, 457, 483, 5, 42, 0, 
		0, 458, 483, 5, 37, 0, 0, 459, 483, 5, 38, 0, 0, 460, 483, 5, 32, 0, 0, 
		461, 483, 5, 28, 0, 0, 462, 483, 5, 43, 0, 0, 463, 483, 5, 50, 0, 0, 464, 
		483, 5, 57, 0, 0, 465, 483, 5, 58, 0, 0, 466, 483, 5, 4, 0, 0, 467, 483, 
		5, 5, 0, 0, 468, 483, 5, 6, 0, 0, 469, 470, 5, 1, 0, 0, 470, 471, 5, 64, 
		0, 0, 471, 472, 7, 11, 0, 0, 472, 483, 5, 65, 0, 0, 473, 483, 3, 86, 43, 
		0, 474, 483, 3, 64, 32, 0, 475, 483, 3, 78, 39, 0, 476, 483, 3, 126, 63, 
		0, 477, 478, 5, 7, 0, 0, 478, 479, 5, 64, 0, 0, 479, 480, 3, 46, 23, 0, 
		480, 481, 5, 65, 0, 0, 481, 483, 1, 0, 0, 0, 482, 455, 1, 0, 0, 0, 482, 
		456, 1, 0, 0, 0, 482, 457, 1, 0, 0, 0, 482, 458, 1, 0, 0, 0, 482, 459, 
		1, 0, 0, 0, 482, 460, 1, 0, 0, 0, 482, 461, 1, 0, 0, 0, 482, 462, 1, 0, 
		0, 0, 482, 463, 1, 0, 0, 0, 482, 464, 1, 0, 0, 0, 482, 465, 1, 0, 0, 0, 
		482, 466, 1, 0, 0, 0, 482, 467, 1, 0, 0, 0, 482, 468, 1, 0, 0, 0, 482, 
		469, 1, 0, 0, 0, 482, 473, 1, 0, 0, 0, 482, 474, 1, 0, 0, 0, 482, 475, 
		1, 0, 0, 0, 482, 476, 1, 0, 0, 0, 482, 477, 1, 0, 0, 0, 483, 63, 1, 0, 
		0, 0, 484, 486, 3, 66, 33, 0, 485, 487, 5, 110, 0, 0, 486, 485, 1, 0, 
		0, 0, 486, 487, 1, 0, 0, 0, 487, 488, 1, 0, 0, 0, 488, 489, 5, 68, 0, 
		0, 489, 490, 3, 68, 34, 0, 490, 491, 5, 69, 0, 0, 491, 496, 1, 0, 0, 0, 
		492, 493, 3, 66, 33, 0, 493, 494, 5, 110, 0, 0, 494, 496, 1, 0, 0, 0, 
		495, 484, 1, 0, 0, 0, 495, 492, 1, 0, 0, 0, 496, 65, 1, 0, 0, 0, 497, 
		498, 7, 12, 0, 0, 498, 67, 1, 0, 0, 0, 499, 501, 3, 70, 35, 0, 500, 499, 
		1, 0, 0, 0, 501, 502, 1, 0, 0, 0, 502, 500, 1, 0, 0, 0, 502, 503, 1, 0, 
		0, 0, 503, 69, 1, 0, 0, 0, 504, 505, 3, 72, 36, 0, 505, 506, 3, 74, 37, 
		0, 506, 507, 5, 92, 0, 0, 507, 513, 1, 0, 0, 0, 508, 509, 3, 72, 36, 0, 
		509, 510, 5, 92, 0, 0, 510, 513, 1, 0, 0, 0, 511, 513, 3, 138, 69, 0, 
		512, 504, 1, 0, 0, 0, 512, 508, 1, 0, 0, 0, 512, 511, 1, 0, 0, 0, 513, 
		71, 1, 0, 0, 0, 514, 517, 3, 62, 31, 0, 515, 517, 3, 88, 44, 0, 516, 514, 
		1, 0, 0, 0, 516, 515, 1, 0, 0, 0, 517, 519, 1, 0, 0, 0, 518, 520, 3, 72, 
		36, 0, 519, 518, 1, 0, 0, 0, 519, 520, 1, 0, 0, 0, 520, 73, 1, 0, 0, 0, 
		521, 526, 3, 76, 38, 0, 522, 523, 5, 93, 0, 0, 523, 525, 3, 76, 38, 0, 
		524, 522, 1, 0, 0, 0, 525, 528, 1, 0, 0, 0, 526, 524, 1, 0, 0, 0, 526, 
		527, 1, 0, 0, 0, 527, 75, 1, 0, 0, 0, 528, 526, 1, 0, 0, 0, 529, 536, 
		3, 94, 47, 0, 530, 532, 3, 94, 47, 0, 531, 530, 1, 0, 0, 0, 531, 532, 
		1, 0, 0, 0, 532, 533, 1, 0, 0, 0, 533, 534, 5, 91, 0, 0, 534, 536, 3, 
		46, 23, 0, 535, 529, 1, 0, 0, 0, 535, 531, 1, 0, 0, 0, 536, 77, 1, 0, 
		0, 0, 537, 539, 5, 30, 0, 0, 538, 540, 5, 110, 0, 0, 539, 538, 1, 0, 0, 
		0, 539, 540, 1, 0, 0, 0, 540, 541, 1, 0, 0, 0, 541, 542, 5, 68, 0, 0, 
		542, 544, 3, 80, 40, 0, 543, 545, 5, 93, 0, 0, 544, 543, 1, 0, 0, 0, 544, 
		545, 1, 0, 0, 0, 545, 546, 1, 0, 0, 0, 546, 547, 5, 69, 0, 0, 547, 551, 
		1, 0, 0, 0, 548, 549, 5, 30, 0, 0, 549, 551, 5, 110, 0, 0, 550, 537, 1, 
		0, 0, 0, 550, 548, 1, 0, 0, 0, 551, 79, 1, 0, 0, 0, 552, 557, 3, 82, 41, 
		0, 553, 554, 5, 93, 0, 0, 554, 556, 3, 82, 41, 0, 555, 553, 1, 0, 0, 0, 
		556, 559, 1, 0, 0, 0, 557, 555, 1, 0, 0, 0, 557, 558, 1, 0, 0, 0, 558, 
		81, 1, 0, 0, 0, 559, 557, 1, 0, 0, 0, 560, 563, 3, 84, 42, 0, 561, 562, 
		5, 94, 0, 0, 562, 564, 3, 46, 23, 0, 563, 561, 1, 0, 0, 0, 563, 564, 1, 
		0, 0, 0, 564, 83, 1, 0, 0, 0, 565, 566, 5, 110, 0, 0, 566, 85, 1, 0, 0, 
		0, 567, 568, 5, 56, 0, 0, 568, 569, 5, 64, 0, 0, 569, 570, 3, 120, 60, 
		0, 570, 571, 5, 65, 0, 0, 571, 87, 1, 0, 0, 0, 572, 573, 7, 13, 0, 0, 
		573, 89, 1, 0, 0, 0, 574, 584, 5, 36, 0, 0, 575, 584, 5, 61, 0, 0, 576, 
		584, 5, 8, 0, 0, 577, 584, 5, 9, 0, 0, 578, 584, 3, 102, 51, 0, 579, 580, 
		5, 10, 0, 0, 580, 581, 5, 64, 0, 0, 581, 582, 5, 110, 0, 0, 582, 584, 
		5, 65, 0, 0, 583, 574, 1, 0, 0, 0, 583, 575, 1, 0, 0, 0, 583, 576, 1, 
		0, 0, 0, 583, 577, 1, 0, 0, 0, 583, 578, 1, 0, 0, 0, 583, 579, 1, 0, 0, 
		0, 584, 91, 1, 0, 0, 0, 585, 586, 5, 54, 0, 0, 586, 589, 5, 64, 0, 0, 
		587, 590, 3, 120, 60, 0, 588, 590, 3, 46, 23, 0, 589, 587, 1, 0, 0, 0, 
		589, 588, 1, 0, 0, 0, 590, 591, 1, 0, 0, 0, 591, 592, 5, 65, 0, 0, 592, 
		93, 1, 0, 0, 0, 593, 595, 3, 108, 54, 0, 594, 593, 1, 0, 0, 0, 594, 595, 
		1, 0, 0, 0, 595, 596, 1, 0, 0, 0, 596, 600, 3, 96, 48, 0, 597, 599, 3, 
		100, 50, 0, 598, 597, 1, 0, 0, 0, 599, 602, 1, 0, 0, 0, 600, 598, 1, 0, 
		0, 0, 600, 601, 1, 0, 0, 0, 601, 95, 1, 0, 0, 0, 602, 600, 1, 0, 0, 0, 
		603, 604, 6, 48, -1, 0, 604, 621, 5, 110, 0, 0, 605, 606, 5, 64, 0, 0, 
		606, 607, 3, 94, 47, 0, 607, 608, 5, 65, 0, 0, 608, 621, 1, 0, 0, 0, 609, 
		610, 5, 110, 0, 0, 610, 611, 5, 91, 0, 0, 611, 621, 5, 112, 0, 0, 612, 
		613, 3, 98, 49, 0, 613, 614, 5, 110, 0, 0, 614, 621, 1, 0, 0, 0, 615, 
		616, 5, 64, 0, 0, 616, 617, 3, 98, 49, 0, 617, 618, 3, 94, 47, 0, 618, 
		619, 5, 65, 0, 0, 619, 621, 1, 0, 0, 0, 620, 603, 1, 0, 0, 0, 620, 605, 
		1, 0, 0, 0, 620, 609, 1, 0, 0, 0, 620, 612, 1, 0, 0, 0, 620, 615, 1, 0, 
		0, 0, 621, 667, 1, 0, 0, 0, 622, 623, 10, 9, 0, 0, 623, 625, 5, 66, 0, 
		0, 624, 626, 3, 110, 55, 0, 625, 624, 1, 0, 0, 0, 625, 626, 1, 0, 0, 0, 
		626, 628, 1, 0, 0, 0, 627, 629, 3, 40, 20, 0, 628, 627, 1, 0, 0, 0, 628, 
		629, 1, 0, 0, 0, 629, 630, 1, 0, 0, 0, 630, 666, 5, 67, 0, 0, 631, 632, 
		10, 8, 0, 0, 632, 633, 5, 66, 0, 0, 633, 635, 5, 45, 0, 0, 634, 636, 3, 
		110, 55, 0, 635, 634, 1, 0, 0, 0, 635, 636, 1, 0, 0, 0, 636, 637, 1, 0, 
		0, 0, 637, 638, 3, 40, 20, 0, 638, 639, 5, 67, 0, 0, 639, 666, 1, 0, 0, 
		0, 640, 641, 10, 7, 0, 0, 641, 642, 5, 66, 0, 0, 642, 643, 3, 110, 55, 
		0, 643, 644, 5, 45, 0, 0, 644, 645, 3, 40, 20, 0, 645, 646, 5, 67, 0, 
		0, 646, 666, 1, 0, 0, 0, 647, 648, 10, 6, 0, 0, 648, 650, 5, 66, 0, 0, 
		649, 651, 3, 110, 55, 0, 650, 649, 1, 0, 0, 0, 650, 651, 1, 0, 0, 0, 651, 
		652, 1, 0, 0, 0, 652, 653, 5, 80, 0, 0, 653, 666, 5, 67, 0, 0, 654, 655, 
		10, 5, 0, 0, 655, 656, 5, 64, 0, 0, 656, 657, 3, 112, 56, 0, 657, 658, 
		5, 65, 0, 0, 658, 666, 1, 0, 0, 0, 659, 660, 10, 4, 0, 0, 660, 662, 5, 
		64, 0, 0, 661, 663, 3, 118, 59, 0, 662, 661, 1, 0, 0, 0, 662, 663, 1, 
		0, 0, 0, 663, 664, 1, 0, 0, 0, 664, 666, 5, 65, 0, 0, 665, 622, 1, 0, 
		0, 0, 665, 631, 1, 0, 0, 0, 665, 640, 1, 0, 0, 0, 665, 647, 1, 0, 0, 0, 
		665, 654, 1, 0, 0, 0, 665, 659, 1, 0, 0, 0, 666, 669, 1, 0, 0, 0, 667, 
		665, 1, 0, 0, 0, 667, 668, 1, 0, 0, 0, 668, 97, 1, 0, 0, 0, 669, 667, 
		1, 0, 0, 0, 670, 671, 7, 14, 0, 0, 671, 99, 1, 0, 0, 0, 672, 673, 5, 16, 
		0, 0, 673, 675, 5, 64, 0, 0, 674, 676, 5, 113, 0, 0, 675, 674, 1, 0, 0, 
		0, 676, 677, 1, 0, 0, 0, 677, 675, 1, 0, 0, 0, 677, 678, 1, 0, 0, 0, 678, 
		679, 1, 0, 0, 0, 679, 682, 5, 65, 0, 0, 680, 682, 3, 102, 51, 0, 681, 
		672, 1, 0, 0, 0, 681, 680, 1, 0, 0, 0, 682, 101, 1, 0, 0, 0, 683, 684, 
		5, 17, 0, 0, 684, 685, 5, 64, 0, 0, 685, 686, 5, 64, 0, 0, 686, 687, 3, 
		104, 52, 0, 687, 688, 5, 65, 0, 0, 688, 689, 5, 65, 0, 0, 689, 103, 1, 
		0, 0, 0, 690, 692, 3, 106, 53, 0, 691, 690, 1, 0, 0, 0, 691, 692, 1, 0, 
		0, 0, 692, 699, 1, 0, 0, 0, 693, 695, 5, 93, 0, 0, 694, 696, 3, 106, 53, 
		0, 695, 694, 1, 0, 0, 0, 695, 696, 1, 0, 0, 0, 696, 698, 1, 0, 0, 0, 697, 
		693, 1, 0, 0, 0, 698, 701, 1, 0, 0, 0, 699, 697, 1, 0, 0, 0, 699, 700, 
		1, 0, 0, 0, 700, 105, 1, 0, 0, 0, 701, 699, 1, 0, 0, 0, 702, 708, 8, 15, 
		0, 0, 703, 705, 5, 64, 0, 0, 704, 706, 3, 10, 5, 0, 705, 704, 1, 0, 0, 
		0, 705, 706, 1, 0, 0, 0, 706, 707, 1, 0, 0, 0, 707, 709, 5, 65, 0, 0, 
		708, 703, 1, 0, 0, 0, 708, 709, 1, 0, 0, 0, 709, 107, 1, 0, 0, 0, 710, 
		712, 7, 16, 0, 0, 711, 713, 3, 110, 55, 0, 712, 711, 1, 0, 0, 0, 712, 
		713, 1, 0, 0, 0, 713, 715, 1, 0, 0, 0, 714, 710, 1, 0, 0, 0, 715, 716, 
		1, 0, 0, 0, 716, 714, 1, 0, 0, 0, 716, 717, 1, 0, 0, 0, 717, 109, 1, 0, 
		0, 0, 718, 720, 3, 88, 44, 0, 719, 718, 1, 0, 0, 0, 720, 721, 1, 0, 0, 
		0, 721, 719, 1, 0, 0, 0, 721, 722, 1, 0, 0, 0, 722, 111, 1, 0, 0, 0, 723, 
		726, 3, 114, 57, 0, 724, 725, 5, 93, 0, 0, 725, 727, 5, 109, 0, 0, 726, 
		724, 1, 0, 0, 0, 726, 727, 1, 0, 0, 0, 727, 113, 1, 0, 0, 0, 728, 733, 
		3, 116, 58, 0, 729, 730, 5, 93, 0, 0, 730, 732, 3, 116, 58, 0, 731, 729, 
		1, 0, 0, 0, 732, 735, 1, 0, 0, 0, 733, 731, 1, 0, 0, 0, 733, 734, 1, 0, 
		0, 0, 734, 115, 1, 0, 0, 0, 735, 733, 1, 0, 0, 0, 736, 737, 3, 50, 25, 
		0, 737, 738, 3, 94, 47, 0, 738, 744, 1, 0, 0, 0, 739, 741, 3, 52, 26, 
		0, 740, 742, 3, 122, 61, 0, 741, 740, 1, 0, 0, 0, 741, 742, 1, 0, 0, 0, 
		742, 744, 1, 0, 0, 0, 743, 736, 1, 0, 0, 0, 743, 739, 1, 0, 0, 0, 744, 
		117, 1, 0, 0, 0, 745, 750, 5, 110, 0, 0, 746, 747, 5, 93, 0, 0, 747, 749, 
		5, 110, 0, 0, 748, 746, 1, 0, 0, 0, 749, 752, 1, 0, 0, 0, 750, 748, 1, 
		0, 0, 0, 750, 751, 1, 0, 0, 0, 751, 119, 1, 0, 0, 0, 752, 750, 1, 0, 0, 
		0, 753, 755, 3, 72, 36, 0, 754, 756, 3, 122, 61, 0, 755, 754, 1, 0, 0, 
		0, 755, 756, 1, 0, 0, 0, 756, 121, 1, 0, 0, 0, 757, 769, 3, 108, 54, 0, 
		758, 760, 3, 108, 54, 0, 759, 758, 1, 0, 0, 0, 759, 760, 1, 0, 0, 0, 760, 
		761, 1, 0, 0, 0, 761, 765, 3, 124, 62, 0, 762, 764, 3, 100, 50, 0, 763, 
		762, 1, 0, 0, 0, 764, 767, 1, 0, 0, 0, 765, 763, 1, 0, 0, 0, 765, 766, 
		1, 0, 0, 0, 766, 769, 1, 0, 0, 0, 767, 765, 1, 0, 0, 0, 768, 757, 1, 0, 
		0, 0, 768, 759, 1, 0, 0, 0, 769, 123, 1, 0, 0, 0, 770, 771, 6, 62, -1, 
		0, 771, 772, 5, 64, 0, 0, 772, 773, 3, 122, 61, 0, 773, 777, 5, 65, 0, 
		0, 774, 776, 3, 100, 50, 0, 775, 774, 1, 0, 0, 0, 776, 779, 1, 0, 0, 0, 
		777, 775, 1, 0, 0, 0, 777, 778, 1, 0, 0, 0, 778, 817, 1, 0, 0, 0, 779, 
		777, 1, 0, 0, 0, 780, 782, 5, 66, 0, 0, 781, 783, 3, 110, 55, 0, 782, 
		781, 1, 0, 0, 0, 782, 783, 1, 0, 0, 0, 783, 785, 1, 0, 0, 0, 784, 786, 
		3, 40, 20, 0, 785, 784, 1, 0, 0, 0, 785, 786, 1, 0, 0, 0, 786, 787, 1, 
		0, 0, 0, 787, 817, 5, 67, 0, 0, 788, 789, 5, 66, 0, 0, 789, 791, 5, 45, 
		0, 0, 790, 792, 3, 110, 55, 0, 791, 790, 1, 0, 0, 0, 791, 792, 1, 0, 0, 
		0, 792, 793, 1, 0, 0, 0, 793, 794, 3, 40, 20, 0, 794, 795, 5, 67, 0, 0, 
		795, 817, 1, 0, 0, 0, 796, 797, 5, 66, 0, 0, 797, 798, 3, 110, 55, 0, 
		798, 799, 5, 45, 0, 0, 799, 800, 3, 40, 20, 0, 800, 801, 5, 67, 0, 0, 
		801, 817, 1, 0, 0, 0, 802, 803, 5, 66, 0, 0, 803, 804, 5, 80, 0, 0, 804, 
		817, 5, 67, 0, 0, 805, 807, 5, 64, 0, 0, 806, 808, 3, 112, 56, 0, 807, 
		806, 1, 0, 0, 0, 807, 808, 1, 0, 0, 0, 808, 809, 1, 0, 0, 0, 809, 813, 
		5, 65, 0, 0, 810, 812, 3, 100, 50, 0, 811, 810, 1, 0, 0, 0, 812, 815, 
		1, 0, 0, 0, 813, 811, 1, 0, 0, 0, 813, 814, 1, 0, 0, 0, 814, 817, 1, 0, 
		0, 0, 815, 813, 1, 0, 0, 0, 816, 770, 1, 0, 0, 0, 816, 780, 1, 0, 0, 0, 
		816, 788, 1, 0, 0, 0, 816, 796, 1, 0, 0, 0, 816, 802, 1, 0, 0, 0, 816, 
		805, 1, 0, 0, 0, 817, 861, 1, 0, 0, 0, 818, 819, 10, 5, 0, 0, 819, 821, 
		5, 66, 0, 0, 820, 822, 3, 110, 55, 0, 821, 820, 1, 0, 0, 0, 821, 822, 
		1, 0, 0, 0, 822, 824, 1, 0, 0, 0, 823, 825, 3, 40, 20, 0, 824, 823, 1, 
		0, 0, 0, 824, 825, 1, 0, 0, 0, 825, 826, 1, 0, 0, 0, 826, 860, 5, 67, 
		0, 0, 827, 828, 10, 4, 0, 0, 828, 829, 5, 66, 0, 0, 829, 831, 5, 45, 0, 
		0, 830, 832, 3, 110, 55, 0, 831, 830, 1, 0, 0, 0, 831, 832, 1, 0, 0, 0, 
		832, 833, 1, 0, 0, 0, 833, 834, 3, 40, 20, 0, 834, 835, 5, 67, 0, 0, 835, 
		860, 1, 0, 0, 0, 836, 837, 10, 3, 0, 0, 837, 838, 5, 66, 0, 0, 838, 839, 
		3, 110, 55, 0, 839, 840, 5, 45, 0, 0, 840, 841, 3, 40, 20, 0, 841, 842, 
		5, 67, 0, 0, 842, 860, 1, 0, 0, 0, 843, 844, 10, 2, 0, 0, 844, 845, 5, 
		66, 0, 0, 845, 846, 5, 80, 0, 0, 846, 860, 5, 67, 0, 0, 847, 848, 10, 
		1, 0, 0, 848, 850, 5, 64, 0, 0, 849, 851, 3, 112, 56, 0, 850, 849, 1, 
		0, 0, 0, 850, 851, 1, 0, 0, 0, 851, 852, 1, 0, 0, 0, 852, 856, 5, 65, 
		0, 0, 853, 855, 3, 100, 50, 0, 854, 853, 1, 0, 0, 0, 855, 858, 1, 0, 0, 
		0, 856, 854, 1, 0, 0, 0, 856, 857, 1, 0, 0, 0, 857, 860, 1, 0, 0, 0, 858, 
		856, 1, 0, 0, 0, 859, 818, 1, 0, 0, 0, 859, 827, 1, 0, 0, 0, 859, 836, 
		1, 0, 0, 0, 859, 843, 1, 0, 0, 0, 859, 847, 1, 0, 0, 0, 860, 863, 1, 0, 
		0, 0, 861, 859, 1, 0, 0, 0, 861, 862, 1, 0, 0, 0, 862, 125, 1, 0, 0, 0, 
		863, 861, 1, 0, 0, 0, 864, 865, 5, 110, 0, 0, 865, 127, 1, 0, 0, 0, 866, 
		875, 3, 40, 20, 0, 867, 868, 5, 68, 0, 0, 868, 870, 3, 130, 65, 0, 869, 
		871, 5, 93, 0, 0, 870, 869, 1, 0, 0, 0, 870, 871, 1, 0, 0, 0, 871, 872, 
		1, 0, 0, 0, 872, 873, 5, 69, 0, 0, 873, 875, 1, 0, 0, 0, 874, 866, 1, 
		0, 0, 0, 874, 867, 1, 0, 0, 0, 875, 129, 1, 0, 0, 0, 876, 878, 3, 132, 
		66, 0, 877, 876, 1, 0, 0, 0, 877, 878, 1, 0, 0, 0, 878, 879, 1, 0, 0, 
		0, 879, 887, 3, 128, 64, 0, 880, 882, 5, 93, 0, 0, 881, 883, 3, 132, 66, 
		0, 882, 881, 1, 0, 0, 0, 882, 883, 1, 0, 0, 0, 883, 884, 1, 0, 0, 0, 884, 
		886, 3, 128, 64, 0, 885, 880, 1, 0, 0, 0, 886, 889, 1, 0, 0, 0, 887, 885, 
		1, 0, 0, 0, 887, 888, 1, 0, 0, 0, 888, 131, 1, 0, 0, 0, 889, 887, 1, 0, 
		0, 0, 890, 891, 3, 134, 67, 0, 891, 892, 5, 94, 0, 0, 892, 133, 1, 0, 
		0, 0, 893, 895, 3, 136, 68, 0, 894, 893, 1, 0, 0, 0, 895, 896, 1, 0, 0, 
		0, 896, 894, 1, 0, 0, 0, 896, 897, 1, 0, 0, 0, 897, 135, 1, 0, 0, 0, 898, 
		899, 5, 66, 0, 0, 899, 900, 3, 46, 23, 0, 900, 901, 5, 67, 0, 0, 901, 
		905, 1, 0, 0, 0, 902, 903, 5, 108, 0, 0, 903, 905, 5, 110, 0, 0, 904, 
		898, 1, 0, 0, 0, 904, 902, 1, 0, 0, 0, 905, 137, 1, 0, 0, 0, 906, 907, 
		5, 62, 0, 0, 907, 908, 5, 64, 0, 0, 908, 909, 3, 46, 23, 0, 909, 911, 
		5, 93, 0, 0, 910, 912, 5, 113, 0, 0, 911, 910, 1, 0, 0, 0, 912, 913, 1, 
		0, 0, 0, 913, 911, 1, 0, 0, 0, 913, 914, 1, 0, 0, 0, 914, 915, 1, 0, 0, 
		0, 915, 916, 5, 65, 0, 0, 916, 917, 5, 92, 0, 0, 917, 139, 1, 0, 0, 0, 
		918, 956, 3, 142, 71, 0, 919, 956, 3, 144, 72, 0, 920, 956, 3, 150, 75, 
		0, 921, 956, 3, 152, 76, 0, 922, 956, 3, 154, 77, 0, 923, 956, 3, 162, 
		81, 0, 924, 925, 7, 17, 0, 0, 925, 926, 7, 18, 0, 0, 926, 935, 5, 64, 
		0, 0, 927, 932, 3, 36, 18, 0, 928, 929, 5, 93, 0, 0, 929, 931, 3, 36, 
		18, 0, 930, 928, 1, 0, 0, 0, 931, 934, 1, 0, 0, 0, 932, 930, 1, 0, 0, 
		0, 932, 933, 1, 0, 0, 0, 933, 936, 1, 0, 0, 0, 934, 932, 1, 0, 0, 0, 935, 
		927, 1, 0, 0, 0, 935, 936, 1, 0, 0, 0, 936, 950, 1, 0, 0, 0, 937, 946, 
		5, 91, 0, 0, 938, 943, 3, 36, 18, 0, 939, 940, 5, 93, 0, 0, 940, 942, 
		3, 36, 18, 0, 941, 939, 1, 0, 0, 0, 942, 945, 1, 0, 0, 0, 943, 941, 1, 
		0, 0, 0, 943, 944, 1, 0, 0, 0, 944, 947, 1, 0, 0, 0, 945, 943, 1, 0, 0, 
		0, 946, 938, 1, 0, 0, 0, 946, 947, 1, 0, 0, 0, 947, 949, 1, 0, 0, 0, 948, 
		937, 1, 0, 0, 0, 949, 952, 1, 0, 0, 0, 950, 948, 1, 0, 0, 0, 950, 951, 
		1, 0, 0, 0, 951, 953, 1, 0, 0, 0, 952, 950, 1, 0, 0, 0, 953, 954, 5, 65, 
		0, 0, 954, 956, 5, 92, 0, 0, 955, 918, 1, 0, 0, 0, 955, 919, 1, 0, 0, 
		0, 955, 920, 1, 0, 0, 0, 955, 921, 1, 0, 0, 0, 955, 922, 1, 0, 0, 0, 955, 
		923, 1, 0, 0, 0, 955, 924, 1, 0, 0, 0, 956, 141, 1, 0, 0, 0, 957, 958, 
		5, 110, 0, 0, 958, 960, 5, 91, 0, 0, 959, 961, 3, 140, 70, 0, 960, 959, 
		1, 0, 0, 0, 960, 961, 1, 0, 0, 0, 961, 971, 1, 0, 0, 0, 962, 963, 5, 22, 
		0, 0, 963, 964, 3, 46, 23, 0, 964, 965, 5, 91, 0, 0, 965, 966, 3, 140, 
		70, 0, 966, 971, 1, 0, 0, 0, 967, 968, 5, 26, 0, 0, 968, 969, 5, 91, 0, 
		0, 969, 971, 3, 140, 70, 0, 970, 957, 1, 0, 0, 0, 970, 962, 1, 0, 0, 0, 
		970, 967, 1, 0, 0, 0, 971, 143, 1, 0, 0, 0, 972, 974, 5, 68, 0, 0, 973, 
		975, 3, 146, 73, 0, 974, 973, 1, 0, 0, 0, 974, 975, 1, 0, 0, 0, 975, 976, 
		1, 0, 0, 0, 976, 977, 5, 69, 0, 0, 977, 145, 1, 0, 0, 0, 978, 980, 3, 
		148, 74, 0, 979, 978, 1, 0, 0, 0, 980, 981, 1, 0, 0, 0, 981, 979, 1, 0, 
		0, 0, 981, 982, 1, 0, 0, 0, 982, 147, 1, 0, 0, 0, 983, 986, 3, 140, 70, 
		0, 984, 986, 3, 48, 24, 0, 985, 983, 1, 0, 0, 0, 985, 984, 1, 0, 0, 0, 
		986, 149, 1, 0, 0, 0, 987, 989, 3, 44, 22, 0, 988, 987, 1, 0, 0, 0, 988, 
		989, 1, 0, 0, 0, 989, 990, 1, 0, 0, 0, 990, 991, 5, 92, 0, 0, 991, 151, 
		1, 0, 0, 0, 992, 993, 5, 35, 0, 0, 993, 994, 5, 64, 0, 0, 994, 995, 3, 
		44, 22, 0, 995, 996, 5, 65, 0, 0, 996, 999, 3, 140, 70, 0, 997, 998, 5, 
		29, 0, 0, 998, 1000, 3, 140, 70, 0, 999, 997, 1, 0, 0, 0, 999, 1000, 1, 
		0, 0, 0, 1000, 1008, 1, 0, 0, 0, 1001, 1002, 5, 47, 0, 0, 1002, 1003, 
		5, 64, 0, 0, 1003, 1004, 3, 44, 22, 0, 1004, 1005, 5, 65, 0, 0, 1005, 
		1006, 3, 140, 70, 0, 1006, 1008, 1, 0, 0, 0, 1007, 992, 1, 0, 0, 0, 1007, 
		1001, 1, 0, 0, 0, 1008, 153, 1, 0, 0, 0, 1009, 1010, 5, 53, 0, 0, 1010, 
		1011, 5, 64, 0, 0, 1011, 1012, 3, 44, 22, 0, 1012, 1013, 5, 65, 0, 0, 
		1013, 1014, 3, 140, 70, 0, 1014, 1030, 1, 0, 0, 0, 1015, 1016, 5, 27, 
		0, 0, 1016, 1017, 3, 140, 70, 0, 1017, 1018, 5, 53, 0, 0, 1018, 1019, 
		5, 64, 0, 0, 1019, 1020, 3, 44, 22, 0, 1020, 1021, 5, 65, 0, 0, 1021, 
		1022, 5, 92, 0, 0, 1022, 1030, 1, 0, 0, 0, 1023, 1024, 5, 33, 0, 0, 1024, 
		1025, 5, 64, 0, 0, 1025, 1026, 3, 156, 78, 0, 1026, 1027, 5, 65, 0, 0, 
		1027, 1028, 3, 140, 70, 0, 1028, 1030, 1, 0, 0, 0, 1029, 1009, 1, 0, 0, 
		0, 1029, 1015, 1, 0, 0, 0, 1029, 1023, 1, 0, 0, 0, 1030, 155, 1, 0, 0, 
		0, 1031, 1036, 3, 158, 79, 0, 1032, 1034, 3, 44, 22, 0, 1033, 1032, 1, 
		0, 0, 0, 1033, 1034, 1, 0, 0, 0, 1034, 1036, 1, 0, 0, 0, 1035, 1031, 1, 
		0, 0, 0, 1035, 1033, 1, 0, 0, 0, 1036, 1037, 1, 0, 0, 0, 1037, 1039, 5, 
		92, 0, 0, 1038, 1040, 3, 160, 80, 0, 1039, 1038, 1, 0, 0, 0, 1039, 1040, 
		1, 0, 0, 0, 1040, 1041, 1, 0, 0, 0, 1041, 1043, 5, 92, 0, 0, 1042, 1044, 
		3, 160, 80, 0, 1043, 1042, 1, 0, 0, 0, 1043, 1044, 1, 0, 0, 0, 1044, 157, 
		1, 0, 0, 0, 1045, 1047, 3, 50, 25, 0, 1046, 1048, 3, 56, 28, 0, 1047, 
		1046, 1, 0, 0, 0, 1047, 1048, 1, 0, 0, 0, 1048, 159, 1, 0, 0, 0, 1049, 
		1054, 3, 40, 20, 0, 1050, 1051, 5, 93, 0, 0, 1051, 1053, 3, 40, 20, 0, 
		1052, 1050, 1, 0, 0, 0, 1053, 1056, 1, 0, 0, 0, 1054, 1052, 1, 0, 0, 0, 
		1054, 1055, 1, 0, 0, 0, 1055, 161, 1, 0, 0, 0, 1056, 1054, 1, 0, 0, 0, 
		1057, 1058, 5, 34, 0, 0, 1058, 1068, 5, 110, 0, 0, 1059, 1068, 5, 25, 
		0, 0, 1060, 1068, 5, 21, 0, 0, 1061, 1063, 5, 41, 0, 0, 1062, 1064, 3, 
		44, 22, 0, 1063, 1062, 1, 0, 0, 0, 1063, 1064, 1, 0, 0, 0, 1064, 1068, 
		1, 0, 0, 0, 1065, 1066, 5, 34, 0, 0, 1066, 1068, 3, 12, 6, 0, 1067, 1057, 
		1, 0, 0, 0, 1067, 1059, 1, 0, 0, 0, 1067, 1060, 1, 0, 0, 0, 1067, 1061, 
		1, 0, 0, 0, 1067, 1065, 1, 0, 0, 0, 1068, 1069, 1, 0, 0, 0, 1069, 1070, 
		5, 92, 0, 0, 1070, 163, 1, 0, 0, 0, 1071, 1073, 3, 166, 83, 0, 1072, 1071, 
		1, 0, 0, 0, 1072, 1073, 1, 0, 0, 0, 1073, 1074, 1, 0, 0, 0, 1074, 1075, 
		5, 0, 0, 1, 1075, 165, 1, 0, 0, 0, 1076, 1078, 3, 168, 84, 0, 1077, 1076, 
		1, 0, 0, 0, 1078, 1079, 1, 0, 0, 0, 1079, 1077, 1, 0, 0, 0, 1079, 1080, 
		1, 0, 0, 0, 1080, 167, 1, 0, 0, 0, 1081, 1085, 3, 170, 85, 0, 1082, 1085, 
		3, 48, 24, 0, 1083, 1085, 5, 92, 0, 0, 1084, 1081, 1, 0, 0, 0, 1084, 1082, 
		1, 0, 0, 0, 1084, 1083, 1, 0, 0, 0, 1085, 169, 1, 0, 0, 0, 1086, 1088, 
		3, 50, 25, 0, 1087, 1086, 1, 0, 0, 0, 1087, 1088, 1, 0, 0, 0, 1088, 1089, 
		1, 0, 0, 0, 1089, 1091, 3, 94, 47, 0, 1090, 1092, 3, 172, 86, 0, 1091, 
		1090, 1, 0, 0, 0, 1091, 1092, 1, 0, 0, 0, 1092, 1093, 1, 0, 0, 0, 1093, 
		1094, 3, 144, 72, 0, 1094, 171, 1, 0, 0, 0, 1095, 1097, 3, 48, 24, 0, 
		1096, 1095, 1, 0, 0, 0, 1097, 1098, 1, 0, 0, 0, 1098, 1096, 1, 0, 0, 0, 
		1098, 1099, 1, 0, 0, 0, 1099, 173, 1, 0, 0, 0, 133, 179, 187, 207, 221, 
		226, 233, 241, 245, 253, 260, 262, 270, 276, 290, 295, 304, 311, 319, 
		327, 335, 343, 351, 359, 367, 375, 383, 392, 400, 409, 416, 421, 426, 
		431, 438, 445, 451, 482, 486, 495, 502, 512, 516, 519, 526, 531, 535, 
		539, 544, 550, 557, 563, 583, 589, 594, 600, 620, 625, 628, 635, 650, 
		662, 665, 667, 677, 681, 691, 695, 699, 705, 708, 712, 716, 721, 726, 
		733, 741, 743, 750, 755, 759, 765, 768, 777, 782, 785, 791, 807, 813, 
		816, 821, 824, 831, 850, 856, 859, 861, 870, 874, 877, 882, 887, 896, 
		904, 913, 932, 935, 943, 946, 950, 955, 960, 970, 974, 981, 985, 988, 
		999, 1007, 1029, 1033, 1035, 1039, 1043, 1047, 1054, 1063, 1067, 1072, 
		1079, 1084, 1087, 1091, 1098
	];
}
