#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statistics {
    #[prost(int64, tag = "1")]
    pub num_rows: i64,
    #[prost(int64, tag = "2")]
    pub total_byte_size: i64,
    #[prost(message, repeated, tag = "3")]
    pub column_stats: ::prost::alloc::vec::Vec<ColumnStats>,
    #[prost(bool, tag = "4")]
    pub is_exact: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileRange {
    #[prost(int64, tag = "1")]
    pub start: i64,
    #[prost(int64, tag = "2")]
    pub end: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionedFile {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub size: u64,
    #[prost(uint64, tag = "3")]
    pub last_modified_ns: u64,
    #[prost(message, repeated, tag = "4")]
    pub partition_values: ::prost::alloc::vec::Vec<
        ::datafusion_proto::protobuf::ScalarValue,
    >,
    #[prost(message, optional, tag = "5")]
    pub range: ::core::option::Option<FileRange>,
}
/// PhysicalPlanNode is a nested type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalPlanNode {
    #[prost(
        oneof = "physical_plan_node::PhysicalPlanType",
        tags = "1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24"
    )]
    pub physical_plan_type: ::core::option::Option<physical_plan_node::PhysicalPlanType>,
}
/// Nested message and enum types in `PhysicalPlanNode`.
pub mod physical_plan_node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PhysicalPlanType {
        #[prost(message, tag = "1")]
        ParquetScan(super::ParquetScanExecNode),
        #[prost(message, tag = "2")]
        CsvScan(super::CsvScanExecNode),
        #[prost(message, tag = "3")]
        Empty(super::EmptyExecNode),
        #[prost(message, tag = "4")]
        Projection(::prost::alloc::boxed::Box<super::ProjectionExecNode>),
        #[prost(message, tag = "6")]
        GlobalLimit(::prost::alloc::boxed::Box<super::GlobalLimitExecNode>),
        #[prost(message, tag = "7")]
        LocalLimit(::prost::alloc::boxed::Box<super::LocalLimitExecNode>),
        #[prost(message, tag = "8")]
        Aggregate(::prost::alloc::boxed::Box<super::AggregateExecNode>),
        #[prost(message, tag = "9")]
        HashJoin(::prost::alloc::boxed::Box<super::HashJoinExecNode>),
        #[prost(message, tag = "10")]
        ShuffleReader(super::ShuffleReaderExecNode),
        #[prost(message, tag = "11")]
        Sort(::prost::alloc::boxed::Box<super::SortExecNode>),
        #[prost(message, tag = "12")]
        CoalesceBatches(::prost::alloc::boxed::Box<super::CoalesceBatchesExecNode>),
        #[prost(message, tag = "13")]
        Filter(::prost::alloc::boxed::Box<super::FilterExecNode>),
        #[prost(message, tag = "14")]
        Merge(::prost::alloc::boxed::Box<super::CoalescePartitionsExecNode>),
        #[prost(message, tag = "15")]
        Unresolved(super::UnresolvedShuffleExecNode),
        #[prost(message, tag = "16")]
        Repartition(::prost::alloc::boxed::Box<super::RepartitionExecNode>),
        #[prost(message, tag = "17")]
        Window(::prost::alloc::boxed::Box<super::WindowAggExecNode>),
        #[prost(message, tag = "18")]
        ShuffleWriter(::prost::alloc::boxed::Box<super::ShuffleWriterExecNode>),
        #[prost(message, tag = "19")]
        CrossJoin(::prost::alloc::boxed::Box<super::CrossJoinExecNode>),
        #[prost(message, tag = "20")]
        AvroScan(super::AvroScanExecNode),
        #[prost(message, tag = "21")]
        Extension(super::PhysicalExtensionNode),
        #[prost(message, tag = "22")]
        Union(super::UnionExecNode),
        #[prost(message, tag = "23")]
        Explain(super::ExplainExecNode),
        #[prost(message, tag = "24")]
        SortPreservingMerge(
            ::prost::alloc::boxed::Box<super::SortPreservingMergeExecNode>,
        ),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalExtensionNode {
    #[prost(bytes = "vec", tag = "1")]
    pub node: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub inputs: ::prost::alloc::vec::Vec<PhysicalPlanNode>,
}
/// physical expressions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalExprNode {
    #[prost(
        oneof = "physical_expr_node::ExprType",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17"
    )]
    pub expr_type: ::core::option::Option<physical_expr_node::ExprType>,
}
/// Nested message and enum types in `PhysicalExprNode`.
pub mod physical_expr_node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExprType {
        /// column references
        #[prost(message, tag = "1")]
        Column(super::PhysicalColumn),
        #[prost(message, tag = "2")]
        Literal(::datafusion_proto::protobuf::ScalarValue),
        /// binary expressions
        #[prost(message, tag = "3")]
        BinaryExpr(::prost::alloc::boxed::Box<super::PhysicalBinaryExprNode>),
        /// aggregate expressions
        #[prost(message, tag = "4")]
        AggregateExpr(super::PhysicalAggregateExprNode),
        /// null checks
        #[prost(message, tag = "5")]
        IsNullExpr(::prost::alloc::boxed::Box<super::PhysicalIsNull>),
        #[prost(message, tag = "6")]
        IsNotNullExpr(::prost::alloc::boxed::Box<super::PhysicalIsNotNull>),
        #[prost(message, tag = "7")]
        NotExpr(::prost::alloc::boxed::Box<super::PhysicalNot>),
        #[prost(message, tag = "8")]
        Case(::prost::alloc::boxed::Box<super::PhysicalCaseNode>),
        #[prost(message, tag = "9")]
        Cast(::prost::alloc::boxed::Box<super::PhysicalCastNode>),
        #[prost(message, tag = "10")]
        Sort(::prost::alloc::boxed::Box<super::PhysicalSortExprNode>),
        #[prost(message, tag = "11")]
        Negative(::prost::alloc::boxed::Box<super::PhysicalNegativeNode>),
        #[prost(message, tag = "12")]
        InList(::prost::alloc::boxed::Box<super::PhysicalInListNode>),
        #[prost(message, tag = "13")]
        ScalarFunction(super::PhysicalScalarFunctionNode),
        #[prost(message, tag = "14")]
        TryCast(::prost::alloc::boxed::Box<super::PhysicalTryCastNode>),
        /// window expressions
        #[prost(message, tag = "15")]
        WindowExpr(::prost::alloc::boxed::Box<super::PhysicalWindowExprNode>),
        #[prost(message, tag = "16")]
        ScalarUdf(super::PhysicalScalarUdfNode),
        #[prost(message, tag = "17")]
        DateTimeIntervalExpr(
            ::prost::alloc::boxed::Box<super::PhysicalDateTimeIntervalExprNode>,
        ),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalScalarUdfNode {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(message, optional, tag = "4")]
    pub return_type: ::core::option::Option<::datafusion_proto::protobuf::ArrowType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalAggregateExprNode {
    #[prost(enumeration = "::datafusion_proto::protobuf::AggregateFunction", tag = "1")]
    pub aggr_function: i32,
    #[prost(message, repeated, tag = "2")]
    pub expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(bool, tag = "3")]
    pub distinct: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalWindowExprNode {
    #[prost(message, optional, boxed, tag = "4")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(oneof = "physical_window_expr_node::WindowFunction", tags = "1, 2")]
    pub window_function: ::core::option::Option<
        physical_window_expr_node::WindowFunction,
    >,
}
/// Nested message and enum types in `PhysicalWindowExprNode`.
pub mod physical_window_expr_node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WindowFunction {
        #[prost(
            enumeration = "::datafusion_proto::protobuf::AggregateFunction",
            tag = "1"
        )]
        AggrFunction(i32),
        /// udaf = 3
        #[prost(
            enumeration = "::datafusion_proto::protobuf::BuiltInWindowFunction",
            tag = "2"
        )]
        BuiltInFunction(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalIsNull {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalIsNotNull {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalNot {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalAliasNode {
    #[prost(message, optional, tag = "1")]
    pub expr: ::core::option::Option<PhysicalExprNode>,
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalBinaryExprNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub l: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub r: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(string, tag = "3")]
    pub op: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalDateTimeIntervalExprNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub l: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub r: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(string, tag = "3")]
    pub op: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalSortExprNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(bool, tag = "2")]
    pub asc: bool,
    #[prost(bool, tag = "3")]
    pub nulls_first: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalWhenThen {
    #[prost(message, optional, tag = "1")]
    pub when_expr: ::core::option::Option<PhysicalExprNode>,
    #[prost(message, optional, tag = "2")]
    pub then_expr: ::core::option::Option<PhysicalExprNode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalInListNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(bool, tag = "3")]
    pub negated: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalCaseNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(message, repeated, tag = "2")]
    pub when_then_expr: ::prost::alloc::vec::Vec<PhysicalWhenThen>,
    #[prost(message, optional, boxed, tag = "3")]
    pub else_expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalScalarFunctionNode {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "::datafusion_proto::protobuf::ScalarFunction", tag = "2")]
    pub fun: i32,
    #[prost(message, repeated, tag = "3")]
    pub args: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(message, optional, tag = "4")]
    pub return_type: ::core::option::Option<::datafusion_proto::protobuf::ArrowType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalTryCastNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(message, optional, tag = "2")]
    pub arrow_type: ::core::option::Option<::datafusion_proto::protobuf::ArrowType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalCastNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
    #[prost(message, optional, tag = "2")]
    pub arrow_type: ::core::option::Option<::datafusion_proto::protobuf::ArrowType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalNegativeNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub expr: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalExprNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnresolvedShuffleExecNode {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
    #[prost(uint32, tag = "3")]
    pub input_partition_count: u32,
    #[prost(uint32, tag = "4")]
    pub output_partition_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, optional, tag = "2")]
    pub expr: ::core::option::Option<PhysicalExprNode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileGroup {
    #[prost(message, repeated, tag = "1")]
    pub files: ::prost::alloc::vec::Vec<PartitionedFile>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanLimit {
    /// wrap into a message to make it optional
    #[prost(uint32, tag = "1")]
    pub limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileScanExecConf {
    #[prost(message, repeated, tag = "1")]
    pub file_groups: ::prost::alloc::vec::Vec<FileGroup>,
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
    #[prost(uint32, repeated, tag = "4")]
    pub projection: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "5")]
    pub limit: ::core::option::Option<ScanLimit>,
    #[prost(message, optional, tag = "6")]
    pub statistics: ::core::option::Option<Statistics>,
    #[prost(string, repeated, tag = "7")]
    pub table_partition_cols: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "8")]
    pub object_store_url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParquetScanExecNode {
    #[prost(message, optional, tag = "1")]
    pub base_conf: ::core::option::Option<FileScanExecConf>,
    #[prost(message, optional, tag = "2")]
    pub pruning_predicate: ::core::option::Option<
        ::datafusion_proto::protobuf::LogicalExprNode,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvScanExecNode {
    #[prost(message, optional, tag = "1")]
    pub base_conf: ::core::option::Option<FileScanExecConf>,
    #[prost(bool, tag = "2")]
    pub has_header: bool,
    #[prost(string, tag = "3")]
    pub delimiter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvroScanExecNode {
    #[prost(message, optional, tag = "1")]
    pub base_conf: ::core::option::Option<FileScanExecConf>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashJoinExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, repeated, tag = "3")]
    pub on: ::prost::alloc::vec::Vec<JoinOn>,
    #[prost(enumeration = "::datafusion_proto::protobuf::JoinType", tag = "4")]
    pub join_type: i32,
    #[prost(enumeration = "PartitionMode", tag = "6")]
    pub partition_mode: i32,
    #[prost(bool, tag = "7")]
    pub null_equals_null: bool,
    #[prost(message, optional, tag = "8")]
    pub filter: ::core::option::Option<JoinFilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnionExecNode {
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<PhysicalPlanNode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainExecNode {
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
    #[prost(message, repeated, tag = "2")]
    pub stringified_plans: ::prost::alloc::vec::Vec<
        ::datafusion_proto::protobuf::StringifiedPlan,
    >,
    #[prost(bool, tag = "3")]
    pub verbose: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossJoinExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub left: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, optional, boxed, tag = "2")]
    pub right: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalColumn {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinOn {
    #[prost(message, optional, tag = "1")]
    pub left: ::core::option::Option<PhysicalColumn>,
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<PhysicalColumn>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyExecNode {
    #[prost(bool, tag = "1")]
    pub produce_one_row: bool,
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectionExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, repeated, tag = "2")]
    pub expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(string, repeated, tag = "3")]
    pub expr_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowAggExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, repeated, tag = "2")]
    pub window_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(string, repeated, tag = "3")]
    pub window_expr_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub input_schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateExecNode {
    #[prost(message, repeated, tag = "1")]
    pub group_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(message, repeated, tag = "2")]
    pub aggr_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(enumeration = "AggregateMode", tag = "3")]
    pub mode: i32,
    #[prost(message, optional, boxed, tag = "4")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(string, repeated, tag = "5")]
    pub group_expr_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub aggr_expr_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// we need the input schema to the partial aggregate to pass to the final aggregate
    #[prost(message, optional, tag = "7")]
    pub input_schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
    #[prost(message, repeated, tag = "8")]
    pub null_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(bool, repeated, tag = "9")]
    pub groups: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuffleWriterExecNode {
    /// TODO it seems redundant to provide job and stage id here since we also have them
    /// in the TaskDefinition that wraps this plan
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub stage_id: u32,
    #[prost(message, optional, boxed, tag = "3")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, optional, tag = "4")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuffleReaderExecNode {
    #[prost(message, repeated, tag = "1")]
    pub partition: ::prost::alloc::vec::Vec<ShuffleReaderPartition>,
    #[prost(message, optional, tag = "2")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuffleReaderPartition {
    /// each partition of a shuffle read can read data from multiple locations
    #[prost(message, repeated, tag = "1")]
    pub location: ::prost::alloc::vec::Vec<PartitionLocation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalLimitExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    /// The number of rows to skip before fetch
    #[prost(uint32, tag = "2")]
    pub skip: u32,
    /// Maximum number of rows to fetch; negative means no limit
    #[prost(int64, tag = "3")]
    pub fetch: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalLimitExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(uint32, tag = "2")]
    pub fetch: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, repeated, tag = "2")]
    pub expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    /// Maximum number of highest/lowest rows to fetch; negative means no limit
    #[prost(int64, tag = "3")]
    pub fetch: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortPreservingMergeExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(message, repeated, tag = "2")]
    pub expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoalesceBatchesExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(uint32, tag = "2")]
    pub target_batch_size: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoalescePartitionsExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalHashRepartition {
    #[prost(message, repeated, tag = "1")]
    pub hash_expr: ::prost::alloc::vec::Vec<PhysicalExprNode>,
    #[prost(uint64, tag = "2")]
    pub partition_count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepartitionExecNode {
    #[prost(message, optional, boxed, tag = "1")]
    pub input: ::core::option::Option<::prost::alloc::boxed::Box<PhysicalPlanNode>>,
    #[prost(oneof = "repartition_exec_node::PartitionMethod", tags = "2, 3, 4")]
    pub partition_method: ::core::option::Option<repartition_exec_node::PartitionMethod>,
}
/// Nested message and enum types in `RepartitionExecNode`.
pub mod repartition_exec_node {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PartitionMethod {
        #[prost(uint64, tag = "2")]
        RoundRobin(u64),
        #[prost(message, tag = "3")]
        Hash(super::PhysicalHashRepartition),
        #[prost(uint64, tag = "4")]
        Unknown(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinFilter {
    #[prost(message, optional, tag = "1")]
    pub expression: ::core::option::Option<PhysicalExprNode>,
    #[prost(message, repeated, tag = "2")]
    pub column_indices: ::prost::alloc::vec::Vec<ColumnIndex>,
    #[prost(message, optional, tag = "3")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnIndex {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(enumeration = "JoinSide", tag = "2")]
    pub side: i32,
}
/// /////////////////////////////////////////////////////////////////////////////////////////////////
/// Ballista Scheduling
/// /////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionGraph {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<JobStatus>,
    #[prost(message, repeated, tag = "4")]
    pub stages: ::prost::alloc::vec::Vec<ExecutionGraphStage>,
    #[prost(uint64, tag = "5")]
    pub output_partitions: u64,
    #[prost(message, repeated, tag = "6")]
    pub output_locations: ::prost::alloc::vec::Vec<PartitionLocation>,
    #[prost(string, tag = "7")]
    pub scheduler_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8")]
    pub task_id_gen: u32,
    #[prost(message, repeated, tag = "9")]
    pub failed_attempts: ::prost::alloc::vec::Vec<StageAttempts>,
    #[prost(string, tag = "10")]
    pub job_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "11")]
    pub start_time: u64,
    #[prost(uint64, tag = "12")]
    pub end_time: u64,
    #[prost(uint64, tag = "13")]
    pub queued_at: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageAttempts {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(uint32, repeated, tag = "2")]
    pub stage_attempt_num: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionGraphStage {
    #[prost(oneof = "execution_graph_stage::StageType", tags = "1, 2, 3, 4")]
    pub stage_type: ::core::option::Option<execution_graph_stage::StageType>,
}
/// Nested message and enum types in `ExecutionGraphStage`.
pub mod execution_graph_stage {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StageType {
        #[prost(message, tag = "1")]
        UnresolvedStage(super::UnResolvedStage),
        #[prost(message, tag = "2")]
        ResolvedStage(super::ResolvedStage),
        #[prost(message, tag = "3")]
        SuccessfulStage(super::SuccessfulStage),
        #[prost(message, tag = "4")]
        FailedStage(super::FailedStage),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnResolvedStage {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(message, optional, tag = "2")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
    #[prost(uint32, repeated, tag = "3")]
    pub output_links: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<GraphStageInput>,
    #[prost(bytes = "vec", tag = "5")]
    pub plan: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "6")]
    pub stage_attempt_num: u32,
    #[prost(string, repeated, tag = "7")]
    pub last_attempt_failure_reasons: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolvedStage {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(uint32, tag = "2")]
    pub partitions: u32,
    #[prost(message, optional, tag = "3")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
    #[prost(uint32, repeated, tag = "4")]
    pub output_links: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "5")]
    pub inputs: ::prost::alloc::vec::Vec<GraphStageInput>,
    #[prost(bytes = "vec", tag = "6")]
    pub plan: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "7")]
    pub stage_attempt_num: u32,
    #[prost(string, repeated, tag = "8")]
    pub last_attempt_failure_reasons: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessfulStage {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(uint32, tag = "2")]
    pub partitions: u32,
    #[prost(message, optional, tag = "3")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
    #[prost(uint32, repeated, tag = "4")]
    pub output_links: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "5")]
    pub inputs: ::prost::alloc::vec::Vec<GraphStageInput>,
    #[prost(bytes = "vec", tag = "6")]
    pub plan: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "7")]
    pub task_infos: ::prost::alloc::vec::Vec<TaskInfo>,
    #[prost(message, repeated, tag = "8")]
    pub stage_metrics: ::prost::alloc::vec::Vec<OperatorMetricsSet>,
    #[prost(uint32, tag = "9")]
    pub stage_attempt_num: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedStage {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(uint32, tag = "2")]
    pub partitions: u32,
    #[prost(message, optional, tag = "3")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
    #[prost(uint32, repeated, tag = "4")]
    pub output_links: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes = "vec", tag = "5")]
    pub plan: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "6")]
    pub task_infos: ::prost::alloc::vec::Vec<TaskInfo>,
    #[prost(message, repeated, tag = "7")]
    pub stage_metrics: ::prost::alloc::vec::Vec<OperatorMetricsSet>,
    #[prost(string, tag = "8")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(uint32, tag = "9")]
    pub stage_attempt_num: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskInfo {
    #[prost(uint32, tag = "1")]
    pub task_id: u32,
    #[prost(uint32, tag = "2")]
    pub partition_id: u32,
    /// Scheduler schedule time
    #[prost(uint64, tag = "3")]
    pub scheduled_time: u64,
    /// Scheduler launch time
    #[prost(uint64, tag = "4")]
    pub launch_time: u64,
    /// The time the Executor start to run the task
    #[prost(uint64, tag = "5")]
    pub start_exec_time: u64,
    /// The time the Executor finish the task
    #[prost(uint64, tag = "6")]
    pub end_exec_time: u64,
    /// Scheduler side finish time
    #[prost(uint64, tag = "7")]
    pub finish_time: u64,
    #[prost(oneof = "task_info::Status", tags = "8, 9, 10")]
    pub status: ::core::option::Option<task_info::Status>,
}
/// Nested message and enum types in `TaskInfo`.
pub mod task_info {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "8")]
        Running(super::RunningTask),
        #[prost(message, tag = "9")]
        Failed(super::FailedTask),
        #[prost(message, tag = "10")]
        Successful(super::SuccessfulTask),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphStageInput {
    #[prost(uint32, tag = "1")]
    pub stage_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub partition_locations: ::prost::alloc::vec::Vec<TaskInputPartitions>,
    #[prost(bool, tag = "3")]
    pub complete: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskInputPartitions {
    #[prost(uint32, tag = "1")]
    pub partition: u32,
    #[prost(message, repeated, tag = "2")]
    pub partition_location: ::prost::alloc::vec::Vec<PartitionLocation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// configuration settings
    #[prost(message, repeated, tag = "100")]
    pub settings: ::prost::alloc::vec::Vec<KeyValuePair>,
    #[prost(oneof = "action::ActionType", tags = "3")]
    pub action_type: ::core::option::Option<action::ActionType>,
}
/// Nested message and enum types in `Action`.
pub mod action {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ActionType {
        /// Fetch a partition from an executor
        #[prost(message, tag = "3")]
        FetchPartition(super::FetchPartition),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutePartition {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub stage_id: u32,
    #[prost(uint32, repeated, tag = "3")]
    pub partition_id: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, optional, tag = "4")]
    pub plan: ::core::option::Option<PhysicalPlanNode>,
    /// The task could need to read partitions from other executors
    #[prost(message, repeated, tag = "5")]
    pub partition_location: ::prost::alloc::vec::Vec<PartitionLocation>,
    /// Output partition for shuffle writer
    #[prost(message, optional, tag = "6")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPartition {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub stage_id: u32,
    #[prost(uint32, tag = "3")]
    pub partition_id: u32,
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub host: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub port: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionLocation {
    /// partition_id of the map stage who produces the shuffle.
    #[prost(uint32, tag = "1")]
    pub map_partition_id: u32,
    /// partition_id of the shuffle, a composition of(job_id + map_stage_id + partition_id).
    #[prost(message, optional, tag = "2")]
    pub partition_id: ::core::option::Option<PartitionId>,
    #[prost(message, optional, tag = "3")]
    pub executor_meta: ::core::option::Option<ExecutorMetadata>,
    #[prost(message, optional, tag = "4")]
    pub partition_stats: ::core::option::Option<PartitionStats>,
    #[prost(string, tag = "5")]
    pub path: ::prost::alloc::string::String,
}
/// Unique identifier for a materialized partition of data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionId {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub stage_id: u32,
    #[prost(uint32, tag = "4")]
    pub partition_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskId {
    #[prost(uint32, tag = "1")]
    pub task_id: u32,
    #[prost(uint32, tag = "2")]
    pub task_attempt_num: u32,
    #[prost(uint32, tag = "3")]
    pub partition_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionStats {
    #[prost(int64, tag = "1")]
    pub num_rows: i64,
    #[prost(int64, tag = "2")]
    pub num_batches: i64,
    #[prost(int64, tag = "3")]
    pub num_bytes: i64,
    #[prost(message, repeated, tag = "4")]
    pub column_stats: ::prost::alloc::vec::Vec<ColumnStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnStats {
    #[prost(message, optional, tag = "1")]
    pub min_value: ::core::option::Option<::datafusion_proto::protobuf::ScalarValue>,
    #[prost(message, optional, tag = "2")]
    pub max_value: ::core::option::Option<::datafusion_proto::protobuf::ScalarValue>,
    #[prost(uint32, tag = "3")]
    pub null_count: u32,
    #[prost(uint32, tag = "4")]
    pub distinct_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorMetricsSet {
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<OperatorMetric>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedCount {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedGauge {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedTime {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorMetric {
    #[prost(oneof = "operator_metric::Metric", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub metric: ::core::option::Option<operator_metric::Metric>,
}
/// Nested message and enum types in `OperatorMetric`.
pub mod operator_metric {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metric {
        #[prost(uint64, tag = "1")]
        OutputRows(u64),
        #[prost(uint64, tag = "2")]
        ElapseTime(u64),
        #[prost(uint64, tag = "3")]
        SpillCount(u64),
        #[prost(uint64, tag = "4")]
        SpilledBytes(u64),
        #[prost(uint64, tag = "5")]
        CurrentMemoryUsage(u64),
        #[prost(message, tag = "6")]
        Count(super::NamedCount),
        #[prost(message, tag = "7")]
        Gauge(super::NamedGauge),
        #[prost(message, tag = "8")]
        Time(super::NamedTime),
        #[prost(int64, tag = "9")]
        StartTimestamp(i64),
        #[prost(int64, tag = "10")]
        EndTimestamp(i64),
    }
}
/// Used by scheduler
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorMetadata {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub port: u32,
    #[prost(uint32, tag = "4")]
    pub grpc_port: u32,
    #[prost(message, optional, tag = "5")]
    pub specification: ::core::option::Option<ExecutorSpecification>,
}
/// Used by grpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorRegistration {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub port: u32,
    #[prost(uint32, tag = "4")]
    pub grpc_port: u32,
    #[prost(message, optional, tag = "5")]
    pub specification: ::core::option::Option<ExecutorSpecification>,
    /// "optional" keyword is stable in protoc 3.15 but prost is still on 3.14 (see <https://github.com/tokio-rs/prost/issues/430> and <https://github.com/tokio-rs/prost/pull/455>)
    /// this syntax is ugly but is binary compatible with the "optional" keyword (see <https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3>)
    #[prost(oneof = "executor_registration::OptionalHost", tags = "2")]
    pub optional_host: ::core::option::Option<executor_registration::OptionalHost>,
}
/// Nested message and enum types in `ExecutorRegistration`.
pub mod executor_registration {
    /// "optional" keyword is stable in protoc 3.15 but prost is still on 3.14 (see <https://github.com/tokio-rs/prost/issues/430> and <https://github.com/tokio-rs/prost/pull/455>)
    /// this syntax is ugly but is binary compatible with the "optional" keyword (see <https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3>)
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalHost {
        #[prost(string, tag = "2")]
        Host(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorHeartbeat {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    /// Unix epoch-based timestamp in seconds
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<ExecutorMetric>,
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<ExecutorStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorMetric {
    /// TODO add more metrics
    #[prost(oneof = "executor_metric::Metric", tags = "1")]
    pub metric: ::core::option::Option<executor_metric::Metric>,
}
/// Nested message and enum types in `ExecutorMetric`.
pub mod executor_metric {
    /// TODO add more metrics
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metric {
        #[prost(uint64, tag = "1")]
        AvailableMemory(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorStatus {
    #[prost(oneof = "executor_status::Status", tags = "1, 2, 3")]
    pub status: ::core::option::Option<executor_status::Status>,
}
/// Nested message and enum types in `ExecutorStatus`.
pub mod executor_status {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(string, tag = "1")]
        Active(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Dead(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Unknown(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorSpecification {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<ExecutorResource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorResource {
    /// TODO add more resources
    #[prost(oneof = "executor_resource::Resource", tags = "1")]
    pub resource: ::core::option::Option<executor_resource::Resource>,
}
/// Nested message and enum types in `ExecutorResource`.
pub mod executor_resource {
    /// TODO add more resources
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(uint32, tag = "1")]
        TaskSlots(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorData {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<ExecutorResourcePair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorResourcePair {
    #[prost(message, optional, tag = "1")]
    pub total: ::core::option::Option<ExecutorResource>,
    #[prost(message, optional, tag = "2")]
    pub available: ::core::option::Option<ExecutorResource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningTask {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedTask {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub retryable: bool,
    /// Whether this task failure should be counted to the maximum number of times the task is allowed to retry
    #[prost(bool, tag = "3")]
    pub count_to_failures: bool,
    #[prost(oneof = "failed_task::FailedReason", tags = "4, 5, 6, 7, 8, 9")]
    pub failed_reason: ::core::option::Option<failed_task::FailedReason>,
}
/// Nested message and enum types in `FailedTask`.
pub mod failed_task {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FailedReason {
        #[prost(message, tag = "4")]
        ExecutionError(super::ExecutionError),
        #[prost(message, tag = "5")]
        FetchPartitionError(super::FetchPartitionError),
        #[prost(message, tag = "6")]
        IoError(super::IoError),
        #[prost(message, tag = "7")]
        ExecutorLost(super::ExecutorLost),
        /// A successful task's result is lost due to executor lost
        #[prost(message, tag = "8")]
        ResultLost(super::ResultLost),
        #[prost(message, tag = "9")]
        TaskKilled(super::TaskKilled),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessfulTask {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    /// TODO tasks are currently always shuffle writes but this will not always be the case
    /// so we might want to think about some refactoring of the task definitions
    #[prost(message, repeated, tag = "2")]
    pub partitions: ::prost::alloc::vec::Vec<ShuffleWritePartition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionError {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPartitionError {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub map_stage_id: u32,
    #[prost(uint32, tag = "3")]
    pub map_partition_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IoError {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorLost {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultLost {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskKilled {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShuffleWritePartition {
    #[prost(uint64, tag = "1")]
    pub partition_id: u64,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub num_batches: u64,
    #[prost(uint64, tag = "4")]
    pub num_rows: u64,
    #[prost(uint64, tag = "5")]
    pub num_bytes: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskStatus {
    #[prost(uint32, tag = "1")]
    pub task_id: u32,
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub stage_id: u32,
    #[prost(uint32, tag = "4")]
    pub stage_attempt_num: u32,
    #[prost(uint32, tag = "5")]
    pub partition_id: u32,
    #[prost(uint64, tag = "6")]
    pub launch_time: u64,
    #[prost(uint64, tag = "7")]
    pub start_exec_time: u64,
    #[prost(uint64, tag = "8")]
    pub end_exec_time: u64,
    #[prost(message, repeated, tag = "12")]
    pub metrics: ::prost::alloc::vec::Vec<OperatorMetricsSet>,
    #[prost(oneof = "task_status::Status", tags = "9, 10, 11")]
    pub status: ::core::option::Option<task_status::Status>,
}
/// Nested message and enum types in `TaskStatus`.
pub mod task_status {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "9")]
        Running(super::RunningTask),
        #[prost(message, tag = "10")]
        Failed(super::FailedTask),
        #[prost(message, tag = "11")]
        Successful(super::SuccessfulTask),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PollWorkParams {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<ExecutorRegistration>,
    #[prost(uint32, tag = "2")]
    pub num_free_slots: u32,
    /// All tasks must be reported until they reach the failed or completed state
    #[prost(message, repeated, tag = "3")]
    pub task_status: ::prost::alloc::vec::Vec<TaskStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskDefinition {
    #[prost(uint32, tag = "1")]
    pub task_id: u32,
    #[prost(uint32, tag = "2")]
    pub task_attempt_num: u32,
    #[prost(string, tag = "3")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub stage_id: u32,
    #[prost(uint32, tag = "5")]
    pub stage_attempt_num: u32,
    #[prost(uint32, tag = "6")]
    pub partition_id: u32,
    #[prost(bytes = "vec", tag = "7")]
    pub plan: ::prost::alloc::vec::Vec<u8>,
    /// Output partition for shuffle writer
    #[prost(message, optional, tag = "8")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
    #[prost(string, tag = "9")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "10")]
    pub launch_time: u64,
    #[prost(message, repeated, tag = "11")]
    pub props: ::prost::alloc::vec::Vec<KeyValuePair>,
}
/// A set of tasks in the same stage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiTaskDefinition {
    #[prost(message, repeated, tag = "1")]
    pub task_ids: ::prost::alloc::vec::Vec<TaskId>,
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub stage_id: u32,
    #[prost(uint32, tag = "4")]
    pub stage_attempt_num: u32,
    #[prost(bytes = "vec", tag = "5")]
    pub plan: ::prost::alloc::vec::Vec<u8>,
    /// Output partition for shuffle writer
    #[prost(message, optional, tag = "6")]
    pub output_partitioning: ::core::option::Option<PhysicalHashRepartition>,
    #[prost(string, tag = "7")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub launch_time: u64,
    #[prost(message, repeated, tag = "9")]
    pub props: ::prost::alloc::vec::Vec<KeyValuePair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSettings {
    #[prost(message, repeated, tag = "1")]
    pub configs: ::prost::alloc::vec::Vec<KeyValuePair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobSessionConfig {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub configs: ::prost::alloc::vec::Vec<KeyValuePair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PollWorkResult {
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<TaskDefinition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterExecutorParams {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<ExecutorRegistration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterExecutorResult {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatParams {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub metrics: ::prost::alloc::vec::Vec<ExecutorMetric>,
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ExecutorStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatResult {
    /// TODO it's from Spark for BlockManager
    #[prost(bool, tag = "1")]
    pub reregister: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopExecutorParams {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    /// stop reason
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    /// force to stop the executor immediately
    #[prost(bool, tag = "3")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopExecutorResult {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorStoppedParams {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    /// stop reason
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutorStoppedResult {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskStatusParams {
    #[prost(string, tag = "1")]
    pub executor_id: ::prost::alloc::string::String,
    /// All tasks must be reported until they reach the failed or completed state
    #[prost(message, repeated, tag = "2")]
    pub task_status: ::prost::alloc::vec::Vec<TaskStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskStatusResult {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQueryParams {
    #[prost(message, repeated, tag = "4")]
    pub settings: ::prost::alloc::vec::Vec<KeyValuePair>,
    #[prost(oneof = "execute_query_params::Query", tags = "1, 2")]
    pub query: ::core::option::Option<execute_query_params::Query>,
    #[prost(oneof = "execute_query_params::OptionalSessionId", tags = "3")]
    pub optional_session_id: ::core::option::Option<
        execute_query_params::OptionalSessionId,
    >,
}
/// Nested message and enum types in `ExecuteQueryParams`.
pub mod execute_query_params {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(bytes, tag = "1")]
        LogicalPlan(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "2")]
        Sql(::prost::alloc::string::String),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalSessionId {
        #[prost(string, tag = "3")]
        SessionId(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteSqlParams {
    #[prost(string, tag = "1")]
    pub sql: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQueryResult {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobStatusParams {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessfulJob {
    #[prost(message, repeated, tag = "1")]
    pub partition_location: ::prost::alloc::vec::Vec<PartitionLocation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuedJob {}
/// TODO: add progress report
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningJob {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailedJob {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatus {
    #[prost(oneof = "job_status::Status", tags = "1, 2, 3, 4")]
    pub status: ::core::option::Option<job_status::Status>,
}
/// Nested message and enum types in `JobStatus`.
pub mod job_status {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "1")]
        Queued(super::QueuedJob),
        #[prost(message, tag = "2")]
        Running(super::RunningJob),
        #[prost(message, tag = "3")]
        Failed(super::FailedJob),
        #[prost(message, tag = "4")]
        Successful(super::SuccessfulJob),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobStatusResult {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<JobStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileMetadataParams {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileMetadataResult {
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<::datafusion_proto::protobuf::Schema>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilePartitionMetadata {
    #[prost(string, repeated, tag = "1")]
    pub filename: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobParams {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobResult {
    #[prost(bool, tag = "1")]
    pub cancelled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanJobDataParams {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanJobDataResult {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchTaskParams {
    /// Allow to launch a task set to an executor at once
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<TaskDefinition>,
    #[prost(string, tag = "2")]
    pub scheduler_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchMultiTaskParams {
    /// Allow to launch a task set to an executor at once
    #[prost(message, repeated, tag = "1")]
    pub multi_tasks: ::prost::alloc::vec::Vec<MultiTaskDefinition>,
    #[prost(string, tag = "2")]
    pub scheduler_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchTaskResult {
    /// TODO when part of the task set are scheduled successfully
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchMultiTaskResult {
    /// TODO when part of the task set are scheduled successfully
    #[prost(bool, tag = "1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTasksParams {
    #[prost(message, repeated, tag = "1")]
    pub task_infos: ::prost::alloc::vec::Vec<RunningTaskInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTasksResult {
    #[prost(bool, tag = "1")]
    pub cancelled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveJobDataParams {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveJobDataResult {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunningTaskInfo {
    #[prost(uint32, tag = "1")]
    pub task_id: u32,
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub stage_id: u32,
    #[prost(uint32, tag = "4")]
    pub partition_id: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartitionMode {
    CollectLeft = 0,
    Partitioned = 1,
    Auto = 2,
}
impl PartitionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PartitionMode::CollectLeft => "COLLECT_LEFT",
            PartitionMode::Partitioned => "PARTITIONED",
            PartitionMode::Auto => "AUTO",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AggregateMode {
    Partial = 0,
    Final = 1,
    FinalPartitioned = 2,
}
impl AggregateMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AggregateMode::Partial => "PARTIAL",
            AggregateMode::Final => "FINAL",
            AggregateMode::FinalPartitioned => "FINAL_PARTITIONED",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JoinSide {
    LeftSide = 0,
    RightSide = 1,
}
impl JoinSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JoinSide::LeftSide => "LEFT_SIDE",
            JoinSide::RightSide => "RIGHT_SIDE",
        }
    }
}
/// Generated client implementations.
pub mod scheduler_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SchedulerGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SchedulerGrpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SchedulerGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SchedulerGrpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SchedulerGrpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Executors must poll the scheduler for heartbeat and to receive tasks
        pub async fn poll_work(
            &mut self,
            request: impl tonic::IntoRequest<super::PollWorkParams>,
        ) -> Result<tonic::Response<super::PollWorkResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/PollWork",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn register_executor(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterExecutorParams>,
        ) -> Result<tonic::Response<super::RegisterExecutorResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/RegisterExecutor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Push-based task scheduler will only leverage this interface
        /// rather than the PollWork interface to report executor states
        pub async fn heart_beat_from_executor(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartBeatParams>,
        ) -> Result<tonic::Response<super::HeartBeatResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/HeartBeatFromExecutor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_task_status(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskStatusParams>,
        ) -> Result<tonic::Response<super::UpdateTaskStatusResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/UpdateTaskStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_file_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileMetadataParams>,
        ) -> Result<tonic::Response<super::GetFileMetadataResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/GetFileMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn execute_query(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteQueryParams>,
        ) -> Result<tonic::Response<super::ExecuteQueryResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/ExecuteQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_job_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobStatusParams>,
        ) -> Result<tonic::Response<super::GetJobStatusResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/GetJobStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Used by Executor to tell Scheduler it is stopped.
        pub async fn executor_stopped(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecutorStoppedParams>,
        ) -> Result<tonic::Response<super::ExecutorStoppedResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/ExecutorStopped",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelJobParams>,
        ) -> Result<tonic::Response<super::CancelJobResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/CancelJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clean_job_data(
            &mut self,
            request: impl tonic::IntoRequest<super::CleanJobDataParams>,
        ) -> Result<tonic::Response<super::CleanJobDataResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.SchedulerGrpc/CleanJobData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod executor_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ExecutorGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExecutorGrpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ExecutorGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ExecutorGrpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ExecutorGrpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn launch_task(
            &mut self,
            request: impl tonic::IntoRequest<super::LaunchTaskParams>,
        ) -> Result<tonic::Response<super::LaunchTaskResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.ExecutorGrpc/LaunchTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn launch_multi_task(
            &mut self,
            request: impl tonic::IntoRequest<super::LaunchMultiTaskParams>,
        ) -> Result<tonic::Response<super::LaunchMultiTaskResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.ExecutorGrpc/LaunchMultiTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stop_executor(
            &mut self,
            request: impl tonic::IntoRequest<super::StopExecutorParams>,
        ) -> Result<tonic::Response<super::StopExecutorResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.ExecutorGrpc/StopExecutor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelTasksParams>,
        ) -> Result<tonic::Response<super::CancelTasksResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.ExecutorGrpc/CancelTasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_job_data(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveJobDataParams>,
        ) -> Result<tonic::Response<super::RemoveJobDataResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ballista.protobuf.ExecutorGrpc/RemoveJobData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod scheduler_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SchedulerGrpcServer.
    #[async_trait]
    pub trait SchedulerGrpc: Send + Sync + 'static {
        /// Executors must poll the scheduler for heartbeat and to receive tasks
        async fn poll_work(
            &self,
            request: tonic::Request<super::PollWorkParams>,
        ) -> Result<tonic::Response<super::PollWorkResult>, tonic::Status>;
        async fn register_executor(
            &self,
            request: tonic::Request<super::RegisterExecutorParams>,
        ) -> Result<tonic::Response<super::RegisterExecutorResult>, tonic::Status>;
        /// Push-based task scheduler will only leverage this interface
        /// rather than the PollWork interface to report executor states
        async fn heart_beat_from_executor(
            &self,
            request: tonic::Request<super::HeartBeatParams>,
        ) -> Result<tonic::Response<super::HeartBeatResult>, tonic::Status>;
        async fn update_task_status(
            &self,
            request: tonic::Request<super::UpdateTaskStatusParams>,
        ) -> Result<tonic::Response<super::UpdateTaskStatusResult>, tonic::Status>;
        async fn get_file_metadata(
            &self,
            request: tonic::Request<super::GetFileMetadataParams>,
        ) -> Result<tonic::Response<super::GetFileMetadataResult>, tonic::Status>;
        async fn execute_query(
            &self,
            request: tonic::Request<super::ExecuteQueryParams>,
        ) -> Result<tonic::Response<super::ExecuteQueryResult>, tonic::Status>;
        async fn get_job_status(
            &self,
            request: tonic::Request<super::GetJobStatusParams>,
        ) -> Result<tonic::Response<super::GetJobStatusResult>, tonic::Status>;
        /// Used by Executor to tell Scheduler it is stopped.
        async fn executor_stopped(
            &self,
            request: tonic::Request<super::ExecutorStoppedParams>,
        ) -> Result<tonic::Response<super::ExecutorStoppedResult>, tonic::Status>;
        async fn cancel_job(
            &self,
            request: tonic::Request<super::CancelJobParams>,
        ) -> Result<tonic::Response<super::CancelJobResult>, tonic::Status>;
        async fn clean_job_data(
            &self,
            request: tonic::Request<super::CleanJobDataParams>,
        ) -> Result<tonic::Response<super::CleanJobDataResult>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SchedulerGrpcServer<T: SchedulerGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SchedulerGrpc> SchedulerGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SchedulerGrpcServer<T>
    where
        T: SchedulerGrpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ballista.protobuf.SchedulerGrpc/PollWork" => {
                    #[allow(non_camel_case_types)]
                    struct PollWorkSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::PollWorkParams>
                    for PollWorkSvc<T> {
                        type Response = super::PollWorkResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PollWorkParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).poll_work(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PollWorkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/RegisterExecutor" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterExecutorSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::RegisterExecutorParams>
                    for RegisterExecutorSvc<T> {
                        type Response = super::RegisterExecutorResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterExecutorParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_executor(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterExecutorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/HeartBeatFromExecutor" => {
                    #[allow(non_camel_case_types)]
                    struct HeartBeatFromExecutorSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::HeartBeatParams>
                    for HeartBeatFromExecutorSvc<T> {
                        type Response = super::HeartBeatResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeartBeatParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).heart_beat_from_executor(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HeartBeatFromExecutorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/UpdateTaskStatus" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTaskStatusSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::UpdateTaskStatusParams>
                    for UpdateTaskStatusSvc<T> {
                        type Response = super::UpdateTaskStatusResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTaskStatusParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_task_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTaskStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/GetFileMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileMetadataSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::GetFileMetadataParams>
                    for GetFileMetadataSvc<T> {
                        type Response = super::GetFileMetadataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFileMetadataParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_file_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFileMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/ExecuteQuery" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteQuerySvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::ExecuteQueryParams>
                    for ExecuteQuerySvc<T> {
                        type Response = super::ExecuteQueryResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteQueryParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).execute_query(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/GetJobStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetJobStatusSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::GetJobStatusParams>
                    for GetJobStatusSvc<T> {
                        type Response = super::GetJobStatusResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobStatusParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_job_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetJobStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/ExecutorStopped" => {
                    #[allow(non_camel_case_types)]
                    struct ExecutorStoppedSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::ExecutorStoppedParams>
                    for ExecutorStoppedSvc<T> {
                        type Response = super::ExecutorStoppedResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecutorStoppedParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).executor_stopped(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecutorStoppedSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/CancelJob" => {
                    #[allow(non_camel_case_types)]
                    struct CancelJobSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::CancelJobParams>
                    for CancelJobSvc<T> {
                        type Response = super::CancelJobResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelJobParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.SchedulerGrpc/CleanJobData" => {
                    #[allow(non_camel_case_types)]
                    struct CleanJobDataSvc<T: SchedulerGrpc>(pub Arc<T>);
                    impl<
                        T: SchedulerGrpc,
                    > tonic::server::UnaryService<super::CleanJobDataParams>
                    for CleanJobDataSvc<T> {
                        type Response = super::CleanJobDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CleanJobDataParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clean_job_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CleanJobDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SchedulerGrpc> Clone for SchedulerGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SchedulerGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SchedulerGrpc> tonic::server::NamedService for SchedulerGrpcServer<T> {
        const NAME: &'static str = "ballista.protobuf.SchedulerGrpc";
    }
}
/// Generated server implementations.
pub mod executor_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExecutorGrpcServer.
    #[async_trait]
    pub trait ExecutorGrpc: Send + Sync + 'static {
        async fn launch_task(
            &self,
            request: tonic::Request<super::LaunchTaskParams>,
        ) -> Result<tonic::Response<super::LaunchTaskResult>, tonic::Status>;
        async fn launch_multi_task(
            &self,
            request: tonic::Request<super::LaunchMultiTaskParams>,
        ) -> Result<tonic::Response<super::LaunchMultiTaskResult>, tonic::Status>;
        async fn stop_executor(
            &self,
            request: tonic::Request<super::StopExecutorParams>,
        ) -> Result<tonic::Response<super::StopExecutorResult>, tonic::Status>;
        async fn cancel_tasks(
            &self,
            request: tonic::Request<super::CancelTasksParams>,
        ) -> Result<tonic::Response<super::CancelTasksResult>, tonic::Status>;
        async fn remove_job_data(
            &self,
            request: tonic::Request<super::RemoveJobDataParams>,
        ) -> Result<tonic::Response<super::RemoveJobDataResult>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ExecutorGrpcServer<T: ExecutorGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExecutorGrpc> ExecutorGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExecutorGrpcServer<T>
    where
        T: ExecutorGrpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ballista.protobuf.ExecutorGrpc/LaunchTask" => {
                    #[allow(non_camel_case_types)]
                    struct LaunchTaskSvc<T: ExecutorGrpc>(pub Arc<T>);
                    impl<
                        T: ExecutorGrpc,
                    > tonic::server::UnaryService<super::LaunchTaskParams>
                    for LaunchTaskSvc<T> {
                        type Response = super::LaunchTaskResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LaunchTaskParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).launch_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LaunchTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.ExecutorGrpc/LaunchMultiTask" => {
                    #[allow(non_camel_case_types)]
                    struct LaunchMultiTaskSvc<T: ExecutorGrpc>(pub Arc<T>);
                    impl<
                        T: ExecutorGrpc,
                    > tonic::server::UnaryService<super::LaunchMultiTaskParams>
                    for LaunchMultiTaskSvc<T> {
                        type Response = super::LaunchMultiTaskResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LaunchMultiTaskParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).launch_multi_task(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LaunchMultiTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.ExecutorGrpc/StopExecutor" => {
                    #[allow(non_camel_case_types)]
                    struct StopExecutorSvc<T: ExecutorGrpc>(pub Arc<T>);
                    impl<
                        T: ExecutorGrpc,
                    > tonic::server::UnaryService<super::StopExecutorParams>
                    for StopExecutorSvc<T> {
                        type Response = super::StopExecutorResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopExecutorParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stop_executor(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopExecutorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.ExecutorGrpc/CancelTasks" => {
                    #[allow(non_camel_case_types)]
                    struct CancelTasksSvc<T: ExecutorGrpc>(pub Arc<T>);
                    impl<
                        T: ExecutorGrpc,
                    > tonic::server::UnaryService<super::CancelTasksParams>
                    for CancelTasksSvc<T> {
                        type Response = super::CancelTasksResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelTasksParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).cancel_tasks(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelTasksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ballista.protobuf.ExecutorGrpc/RemoveJobData" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveJobDataSvc<T: ExecutorGrpc>(pub Arc<T>);
                    impl<
                        T: ExecutorGrpc,
                    > tonic::server::UnaryService<super::RemoveJobDataParams>
                    for RemoveJobDataSvc<T> {
                        type Response = super::RemoveJobDataResult;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveJobDataParams>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_job_data(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveJobDataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ExecutorGrpc> Clone for ExecutorGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ExecutorGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExecutorGrpc> tonic::server::NamedService for ExecutorGrpcServer<T> {
        const NAME: &'static str = "ballista.protobuf.ExecutorGrpc";
    }
}
