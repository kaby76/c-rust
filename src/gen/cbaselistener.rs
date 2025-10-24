// Generated from C.g4 by ANTLR 4.13.2

use super::cparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by CParser.

pub trait CBaseListener<'input>:
    ParseTreeListener<'input, CParserContextType> {

    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_primaryexpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_primaryexpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_genericselection(&mut self, _ctx: &GenericSelectionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_genericselection(&mut self, _ctx: &GenericSelectionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_genericassoclist(&mut self, _ctx: &GenericAssocListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_genericassoclist(&mut self, _ctx: &GenericAssocListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_genericassociation(&mut self, _ctx: &GenericAssociationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_genericassociation(&mut self, _ctx: &GenericAssociationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfixexpression(&mut self, _ctx: &PostfixExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfixexpression(&mut self, _ctx: &PostfixExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_argumentexpressionlist(&mut self, _ctx: &ArgumentExpressionListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_argumentexpressionlist(&mut self, _ctx: &ArgumentExpressionListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unaryexpression(&mut self, _ctx: &UnaryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unaryexpression(&mut self, _ctx: &UnaryExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unaryoperator(&mut self, _ctx: &UnaryOperatorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unaryoperator(&mut self, _ctx: &UnaryOperatorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_castexpression(&mut self, _ctx: &CastExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_castexpression(&mut self, _ctx: &CastExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multiplicativeexpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multiplicativeexpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_additiveexpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_additiveexpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_shiftexpression(&mut self, _ctx: &ShiftExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_shiftexpression(&mut self, _ctx: &ShiftExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_relationalexpression(&mut self, _ctx: &RelationalExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_relationalexpression(&mut self, _ctx: &RelationalExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_equalityexpression(&mut self, _ctx: &EqualityExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_equalityexpression(&mut self, _ctx: &EqualityExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_andexpression(&mut self, _ctx: &AndExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_andexpression(&mut self, _ctx: &AndExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_exclusiveorexpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_exclusiveorexpression(&mut self, _ctx: &ExclusiveOrExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inclusiveorexpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inclusiveorexpression(&mut self, _ctx: &InclusiveOrExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_logicalandexpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_logicalandexpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_logicalorexpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_logicalorexpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_conditionalexpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_conditionalexpression(&mut self, _ctx: &ConditionalExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_assignmentexpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assignmentexpression(&mut self, _ctx: &AssignmentExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_assignmentoperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assignmentoperator(&mut self, _ctx: &AssignmentOperatorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_constantexpression(&mut self, _ctx: &ConstantExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_constantexpression(&mut self, _ctx: &ConstantExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declarationspecifiers(&mut self, _ctx: &DeclarationSpecifiersContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declarationspecifiers(&mut self, _ctx: &DeclarationSpecifiersContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declarationspecifiers2(&mut self, _ctx: &DeclarationSpecifiers2Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declarationspecifiers2(&mut self, _ctx: &DeclarationSpecifiers2Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declarationspecifier(&mut self, _ctx: &DeclarationSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declarationspecifier(&mut self, _ctx: &DeclarationSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_initdeclaratorlist(&mut self, _ctx: &InitDeclaratorListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_initdeclaratorlist(&mut self, _ctx: &InitDeclaratorListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_initdeclarator(&mut self, _ctx: &InitDeclaratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_initdeclarator(&mut self, _ctx: &InitDeclaratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_storageclassspecifier(&mut self, _ctx: &StorageClassSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_storageclassspecifier(&mut self, _ctx: &StorageClassSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typespecifier(&mut self, _ctx: &TypeSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typespecifier(&mut self, _ctx: &TypeSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structorunionspecifier(&mut self, _ctx: &StructOrUnionSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structorunionspecifier(&mut self, _ctx: &StructOrUnionSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structorunion(&mut self, _ctx: &StructOrUnionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structorunion(&mut self, _ctx: &StructOrUnionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structdeclarationlist(&mut self, _ctx: &StructDeclarationListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structdeclarationlist(&mut self, _ctx: &StructDeclarationListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structdeclaration(&mut self, _ctx: &StructDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structdeclaration(&mut self, _ctx: &StructDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_specifierqualifierlist(&mut self, _ctx: &SpecifierQualifierListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_specifierqualifierlist(&mut self, _ctx: &SpecifierQualifierListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structdeclaratorlist(&mut self, _ctx: &StructDeclaratorListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structdeclaratorlist(&mut self, _ctx: &StructDeclaratorListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structdeclarator(&mut self, _ctx: &StructDeclaratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structdeclarator(&mut self, _ctx: &StructDeclaratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumspecifier(&mut self, _ctx: &EnumSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumspecifier(&mut self, _ctx: &EnumSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumeratorlist(&mut self, _ctx: &EnumeratorListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumeratorlist(&mut self, _ctx: &EnumeratorListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumerator(&mut self, _ctx: &EnumeratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumerator(&mut self, _ctx: &EnumeratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumerationconstant(&mut self, _ctx: &EnumerationConstantContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumerationconstant(&mut self, _ctx: &EnumerationConstantContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atomictypespecifier(&mut self, _ctx: &AtomicTypeSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atomictypespecifier(&mut self, _ctx: &AtomicTypeSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typequalifier(&mut self, _ctx: &TypeQualifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typequalifier(&mut self, _ctx: &TypeQualifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functionspecifier(&mut self, _ctx: &FunctionSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functionspecifier(&mut self, _ctx: &FunctionSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_alignmentspecifier(&mut self, _ctx: &AlignmentSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_alignmentspecifier(&mut self, _ctx: &AlignmentSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declarator(&mut self, _ctx: &DeclaratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declarator(&mut self, _ctx: &DeclaratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_directdeclarator(&mut self, _ctx: &DirectDeclaratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_directdeclarator(&mut self, _ctx: &DirectDeclaratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_vcspecificmodifer(&mut self, _ctx: &VcSpecificModiferContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_vcspecificmodifer(&mut self, _ctx: &VcSpecificModiferContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gccdeclaratorextension(&mut self, _ctx: &GccDeclaratorExtensionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gccdeclaratorextension(&mut self, _ctx: &GccDeclaratorExtensionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gccattributespecifier(&mut self, _ctx: &GccAttributeSpecifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gccattributespecifier(&mut self, _ctx: &GccAttributeSpecifierContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gccattributelist(&mut self, _ctx: &GccAttributeListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gccattributelist(&mut self, _ctx: &GccAttributeListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_gccattribute(&mut self, _ctx: &GccAttributeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_gccattribute(&mut self, _ctx: &GccAttributeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_pointer(&mut self, _ctx: &PointerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pointer(&mut self, _ctx: &PointerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typequalifierlist(&mut self, _ctx: &TypeQualifierListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typequalifierlist(&mut self, _ctx: &TypeQualifierListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parametertypelist(&mut self, _ctx: &ParameterTypeListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parametertypelist(&mut self, _ctx: &ParameterTypeListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parameterlist(&mut self, _ctx: &ParameterListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parameterlist(&mut self, _ctx: &ParameterListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parameterdeclaration(&mut self, _ctx: &ParameterDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parameterdeclaration(&mut self, _ctx: &ParameterDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_identifierlist(&mut self, _ctx: &IdentifierListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_identifierlist(&mut self, _ctx: &IdentifierListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typename(&mut self, _ctx: &TypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typename(&mut self, _ctx: &TypeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abstractdeclarator(&mut self, _ctx: &AbstractDeclaratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abstractdeclarator(&mut self, _ctx: &AbstractDeclaratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_directabstractdeclarator(&mut self, _ctx: &DirectAbstractDeclaratorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_directabstractdeclarator(&mut self, _ctx: &DirectAbstractDeclaratorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typedefname(&mut self, _ctx: &TypedefNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typedefname(&mut self, _ctx: &TypedefNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_initializer(&mut self, _ctx: &InitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_initializer(&mut self, _ctx: &InitializerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_initializerlist(&mut self, _ctx: &InitializerListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_initializerlist(&mut self, _ctx: &InitializerListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_designation(&mut self, _ctx: &DesignationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_designation(&mut self, _ctx: &DesignationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_designatorlist(&mut self, _ctx: &DesignatorListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_designatorlist(&mut self, _ctx: &DesignatorListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_designator(&mut self, _ctx: &DesignatorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_designator(&mut self, _ctx: &DesignatorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_staticassertdeclaration(&mut self, _ctx: &StaticAssertDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_staticassertdeclaration(&mut self, _ctx: &StaticAssertDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_labeledstatement(&mut self, _ctx: &LabeledStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_labeledstatement(&mut self, _ctx: &LabeledStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compoundstatement(&mut self, _ctx: &CompoundStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compoundstatement(&mut self, _ctx: &CompoundStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_blockitemlist(&mut self, _ctx: &BlockItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_blockitemlist(&mut self, _ctx: &BlockItemListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_blockitem(&mut self, _ctx: &BlockItemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_blockitem(&mut self, _ctx: &BlockItemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expressionstatement(&mut self, _ctx: &ExpressionStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expressionstatement(&mut self, _ctx: &ExpressionStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_selectionstatement(&mut self, _ctx: &SelectionStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_selectionstatement(&mut self, _ctx: &SelectionStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_iterationstatement(&mut self, _ctx: &IterationStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_iterationstatement(&mut self, _ctx: &IterationStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_forcondition(&mut self, _ctx: &ForConditionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_forcondition(&mut self, _ctx: &ForConditionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fordeclaration(&mut self, _ctx: &ForDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fordeclaration(&mut self, _ctx: &ForDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_forexpression(&mut self, _ctx: &ForExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_forexpression(&mut self, _ctx: &ForExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_jumpstatement(&mut self, _ctx: &JumpStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_jumpstatement(&mut self, _ctx: &JumpStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compilationunit(&mut self, _ctx: &CompilationUnitContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compilationunit(&mut self, _ctx: &CompilationUnitContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_translationunit(&mut self, _ctx: &TranslationUnitContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_translationunit(&mut self, _ctx: &TranslationUnitContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_externaldeclaration(&mut self, _ctx: &ExternalDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_externaldeclaration(&mut self, _ctx: &ExternalDeclarationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functiondefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functiondefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declarationlist(&mut self, _ctx: &DeclarationListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  CBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declarationlist(&mut self, _ctx: &DeclarationListContext<'input>) {}


}