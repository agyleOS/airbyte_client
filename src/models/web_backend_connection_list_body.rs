#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WebBackendConnectionListBody {
    #[serde(rename = "workspaceId")]
    pub workspace_id: uuid::Uuid,
    #[serde(rename = "sourceId")]
    pub source_id: Option<Vec<uuid::Uuid>>,
    #[serde(rename = "destinationId")]
    pub destination_id: Option<Vec<uuid::Uuid>>,
}

impl WebBackendConnectionListBody {
    pub fn new(workspace_id: uuid::Uuid, source_id: Option<Vec<uuid::Uuid>>, destination_id: Option<Vec<uuid::Uuid>>) -> WebBackendConnectionListBody {
        WebBackendConnectionListBody { workspace_id, source_id, destination_id }
    }
}
