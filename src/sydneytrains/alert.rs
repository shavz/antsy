struct AlertEntity {
    id: u16,
    alert: Alert,

}

struct InformedEntity{
    agency_id: String,
    route_id: String,
}

struct Alert {
    informed_entity: InformedEntity,
    description_text: AlertDescription,
}

struct AlertDescription {
    translation: AlertDescriptionTextTranslation
}

struct AlertDescriptionTextTranslation {
    text: String,
    language: String
}
