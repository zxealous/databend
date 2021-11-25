// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_datavalues::prelude::*;
use common_exception::Result;
use common_planners::*;

#[test]
fn test_projection_plan() -> Result<()> {
    use pretty_assertions::assert_eq;

    let schema = DataSchemaRefExt::create(vec![DataField::new("a", DataType::String, false)]);
    let empty_plan = EmptyPlan::create_with_schema(schema);

    let projection = PlanNode::Projection(ProjectionPlan {
        expr: vec![col("a")],
        schema: DataSchemaRefExt::create(vec![DataField::new("a", DataType::String, false)]),
        input: Arc::from(PlanBuilder::from(&PlanNode::Empty(empty_plan)).build()?),
    });
    let _ = projection.schema();
    let expect = "Projection: a:String";
    let actual = format!("{:?}", projection);
    assert_eq!(expect, actual);
    Ok(())
}