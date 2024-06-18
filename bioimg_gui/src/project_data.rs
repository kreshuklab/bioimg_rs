
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthorWidgetRawData{
    pub name_widget: String,
    pub affiliation_widget: Option<String>,
    pub email_widget: Option<String>,
    pub github_user_widget: Option<String>,
    pub orcid_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CiteEntryWidgetRawData {
    pub citation_text_widget: String,
    pub doi_widget: Option<String>,
    pub url_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StagingMaintainerRawData {
    pub github_user_widget: String,
    pub affiliation_widget: Option<String>,
    pub email_widget: Option<String>,
    pub orcid_widget: Option<String>,
    pub name_widget: Option<String>,
}

pub struct ProjectData1{
    pub staging_name: String,
    pub staging_description: String,
    // pub cover_images: Vec<>, // something from LoadingSate in image_widget_2
    // id?
    // pub staging_authors: Vec<CollapsibleWidget<StagingAuthor2>>,
    // pub attachments_widget: StagingVec<AttachmentsWidget>,
    // pub staging_citations: StagingVec<CollapsibleWidget<StagingCiteEntry2>>,
    // //config
    // pub staging_git_repo: StagingOpt<StagingUrl>,
    // pub icon_widget: StagingOpt<IconWidget>,
    // //links
    // pub staging_maintainers: StagingVec<CollapsibleWidget<StagingMaintainer>>,
    // pub staging_tags: StagingVec<StagingString<rdf::Tag>>,
    // pub staging_version: StagingOpt<VersionWidget>,

    // pub staging_documentation: CodeEditorWidget,
    // pub staging_license: SearchAndPickWidget<rdf::LicenseId>,
    // //badges
    // pub model_interface_widget: ModelInterfaceWidget,
    // ////
    // pub weights_widget: WeightsWidget,

    // pub notifications_widget: NotificationsWidget,
    // model_packing_status: PackingStatus,
}
