// cargo run --example pageable_policy_insights_mgmt
// TODO:
// https://github.com/Azure/azure-rest-api-specs/tree/master/specification/resources/resource-manager

use autorust_codegen::{autorust_toml::PackageConfig, *};

fn main() -> Result<()> {
    let output_folder = "../../../azure-sdk-for-rust/services/mgmt/policyinsights/src/package_2023_03";
    let input_files = [
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/preview/2018-07-01-preview/policyTrackedResources.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2021-10-01/remediations.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2019-10-01/policyEvents.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2019-10-01/policyStates.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2019-10-01/policyMetadata.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2023-03-01/checkPolicyRestrictions.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2022-04-01/componentPolicyStates.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2022-04-01/operations.json",
        "../../../azure-rest-api-specs/specification/policyinsights/resource-manager/Microsoft.PolicyInsights/stable/2022-09-01/attestations.json"
        ];
    run(
        &CrateConfig {
            run_config: &RunConfig::new("azure_mgmt_"),
            output_folder: output_folder.into(),
            input_files: input_files.iter().map(Into::into).collect(),
        },
        &PackageConfig::default(),
    )?;
    Ok(())
}
