macro_rules! core_gate {
    (@apply $condition:meta; $($item:item)*) => {
        $(#[cfg($condition)] $item)*
    };
    (page_streams { $($item:item)* }) => { core_gate!(@apply any(
            feature = "admin", feature = "agile", feature = "assets", feature = "assets-server",
            feature = "cloud", feature = "server", feature = "service-desk", feature = "service-desk-server",
            feature = "teams", feature = "user-provisioning"
        ); $($item)*); };
    (cursor_pages { $($item:item)* }) => { core_gate!(@apply any(feature = "admin", feature = "agile", feature = "cloud", feature = "server", feature = "teams"); $($item)*); };
    (offset_pages { $($item:item)* }) => { core_gate!(@apply any(
            feature = "agile", feature = "assets", feature = "assets-server", feature = "cloud",
            feature = "server", feature = "service-desk", feature = "service-desk-server"
        ); $($item)*); };
    (offset_from_pages { $($item:item)* }) => { core_gate!(@apply any(
            feature = "agile", feature = "assets", feature = "assets-server", feature = "cloud",
            feature = "server", feature = "service-desk", feature = "service-desk-server", feature = "user-provisioning"
        ); $($item)*); };
    (path_segments { $($item:item)* }) => { core_gate!(@apply any(
            feature = "admin", feature = "agile", feature = "assets", feature = "assets-server",
            feature = "cloud", feature = "server", feature = "service-desk", feature = "service-desk-server",
            feature = "teams", feature = "user-management", feature = "user-provisioning"
        ); $($item)*); };
    (optional_dates { $($item:item)* }) => { core_gate!(@apply any(
            feature = "admin", feature = "agile", feature = "assets", feature = "assets-server",
            feature = "cloud", feature = "server", feature = "service-desk", feature = "service-desk-server",
            feature = "user-provisioning"
        ); $($item)*); };
    (required_dates { $($item:item)* }) => { core_gate!(@apply any(feature = "admin", feature = "agile", feature = "assets", feature = "cloud"); $($item)*); };
    (jira_datetime_serialization { $($item:item)* }) => { core_gate!(@apply any(
            feature = "admin", feature = "agile", feature = "cloud", feature = "server",
            feature = "service-desk", feature = "service-desk-server", feature = "user-provisioning"
        ); $($item)*); };
    (rfc3339_datetime_serialization { $($item:item)* }) => { core_gate!(@apply any(feature = "assets", feature = "assets-server"); $($item)*); };
    (untagged_variants { $($item:item)* }) => { core_gate!(@apply any(
            feature = "admin", feature = "agile", feature = "assets", feature = "cloud",
            feature = "server", feature = "user-management"
        ); $($item)*); };
}
