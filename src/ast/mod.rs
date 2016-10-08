use std::fmt::{Debug, Formatter, Error};

pub enum Stmt {
    AlterTable {
        tbl_name: QualifiedName,
        body: AlterTableBody,
    },
    Analyze { obj_name: Option<QualifiedName> },
    Attach {
        expr: Expr,
        db_name: Expr,
        key: Option<Expr>,
    },
    Begin {
        tx_type: Option<TransactionType>,
        tx_name: Option<String>,
    },
    Commit { tx_name: Option<String> },
    CreateIndex {
        unique: bool,
        if_not_exists: bool,
        idx_name: QualifiedName,
        tbl_name: String,
        columns: Vec<SortedColumn>,
        where_clause: Option<Where>,
    },
    CreateTable {
        temporary: bool,
        if_not_exists: bool,
        tbl_name: QualifiedName,
        body: CreateTableBody,
    },
    CreateTrigger {}, // TODO
    CreateView {}, // TODO
    CreateVirtualTable {}, // TODO
    Delete {}, // TODO
    Detach { db_name: Expr },
    DropIndex {}, // TODO
    DropTable {}, // TODO
    DropTrigger {}, // TODO
    DropView {}, // TODO
    Insert {}, // TODO
    Pragma {
        name: QualifiedName,
        body: Option<PragmaBody>,
    },
    Reindex { obj_name: Option<QualifiedName> },
    Release { savepoint_name: Option<String> },
    Rollback {
        tx_name: Option<String>,
        savepoint_name: Option<String>,
    },
    Savepoint { savepoint_name: Option<String> },
    Select(Select),
    Update {}, // TODO
    Vacuum { db_name: Option<String> },
}

// TODO
pub struct Expr;
pub struct Select;
pub struct Where;

pub struct QualifiedName {
    db_name: Option<String>,
    name: String,
}

pub enum AlterTableBody {
    RenameTo { tbl_name: String },
    AddColumn(ColumnDefinition),
}

pub enum CreateTableBody {
    ColumnsAndConstraints {
        columns: Vec<ColumnDefinition>,
        constraints: Option<Vec<NamedTableConstraint>>,
        without: Option<String>,
    },
    AsSelect(Select),
}

pub struct ColumnDefinition {
    col_name: String,
    col_type: Option<Type>,
    constraints: Vec<NamedColumnConstraint>,
}

pub struct NamedColumnConstraint {
    name: Option<String>,
    constraint: ColumnConstraint,
}

pub enum ColumnConstraint {
    PrimaryKey {
        order: Option<SortOrder>,
        conflict_clause: Option<ConflictClause>,
        auto_increment: bool,
    },
    NotNull {
        nullable: bool,
        conflict_clause: Option<ConflictClause>,
    },
    Unique(Option<ConflictClause>),
    Check(Expr),
    Default(DefaultValue),
    Collate { collation_name: String },
    ForeignKey(ForeignKeyClause),
}

pub struct NamedTableConstraint {
    name: Option<String>,
    constraint: TableConstraint,
}

pub enum TableConstraint {
    PrimaryKey {
        columns: Vec<SortedColumn>,
        conflict_clause: Option<ConflictClause>,
        auto_increment: bool,
    },
    Unique {
        columns: Vec<SortedColumn>,
        conflict_clause: Option<ConflictClause>,
    },
    Check(Expr),
    ForeignKey {
        columns: Vec<IndexedColumn>,
        clause: ForeignKeyClause,
        deref_clause: DeferSubclause,
    },
}

pub enum SortOrder {
    Asc,
    Desc,
}

pub enum ConflictClause {
    OnConflictRollback,
    OnConflictAbort,
    OnConflictFail,
    OnConflictIgnore,
    OnConflictReplace,
}

pub enum DefaultValue {
    Expr(Expr), // TODO
}

pub struct ForeignKeyClause {
    tbl_name: String,
    columns: Option<Vec<IndexedColumn>>,
    args: Option<Vec<RefArgs>>,
}

pub enum RefArgs {
    OnDelete(RefAct),
    OnUpdate(RefAct),
    Match { name: String },
}

pub enum RefAct {
    SetNull,
    SetDefault,
    Cascade,
    Restrict,
    NoAction,
}

pub struct DeferSubclause {
    deferrable: bool,
    init_deferred: Option<InitDeferredPred>,
}

pub enum InitDeferredPred {
    InitiallyDeferred,
    InitiallyImmediate,
}

pub struct IndexedColumn {
    col_name: String,
    collation_name: Option<String>,
    order: Option<SortOrder>,
}

pub struct SortedColumn {
    expr: Expr,
    order: Option<SortOrder>,
}

pub enum PragmaBody {
    Equals(PragmaValue),
    Call(PragmaValue),
}

pub type PragmaValue = String; // TODO

pub struct Type {
    name: String, // TODO Validate
    size: Option<TypeSize>,
}

pub enum TypeSize {
    MaxSize(String),
    TypeSize(String, String),
}

#[derive(Copy, Clone)]
pub enum TransactionType {
    Deferred,
    Immediate,
    Exclusive,
}

impl Debug for TransactionType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::TransactionType::*;
        match *self {
            Deferred => write!(fmt, "DEFERRED"),
            Immediate => write!(fmt, "IMMEDIATE"),
            Exclusive => write!(fmt, "EXCLUSIVE"),
        }
    }
}
