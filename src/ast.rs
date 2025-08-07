use either::Either;

pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct Path<'a> {
    pub segments: Vec<PathSegment<'a>>,
    pub trailing_slash: bool,
    pub span: Span,
}

pub enum PathSegment<'a> {
    Simple(&'a str),
    MetaVar(&'a str),
}

pub struct File<'a> {
    pub content: Vec<Function<'a>>
}

pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<&'a str>,
    pub watched_inputs: Vec<Path<'a>>,
    pub watched_outputs: Vec<Path<'a>>,
    pub body: Vec<Statement<'a>>,
    pub span: Span
}

pub enum Statement<'a> {
    Assignment(&'a str, Expression<'a>),
    ListUnpackingAssignment {
        element_variables: Vec<&'a str>,
        rest_variable: Option<&'a str>,
        expr: Expression<'a>,
    },
    IfStatement(IfStatement<'a>),
    CommandStatement(Vec<Expression<'a>>),
    Task {
        directories: Vec<Path<'a>>,
        body: Vec<Statement<'a>>,
    }
}

pub struct IfStatement<'a> {
    test: Expression<'a>,
    body: Vec<Statement<'a>>,
    else_body: Option<Either<Vec<Statement<'a>>, Box<IfStatement<'a>>>>,
}

pub enum Expression<'a> {
    String(StringExpr<'a>),
    Int(&'a str),
    Float(&'a str),
    Boolean(bool),
    List(Vec<Expression<'a>>),
    Dictionary(Vec<(Expression<'a>, Expression<'a>)>),
    Variable(&'a str),
    FunctionCall {
        name: &'a str,
        arguments: Vec<Expression<'a>>,
    }
}

pub struct StringExpr<'a> {
    pub parts: Vec<StringPart<'a>>,
}

pub enum StringPart<'a> {
    String(&'a str),
    Interpolation(Statement<'a>),
}