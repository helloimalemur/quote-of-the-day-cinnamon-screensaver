struct QOD {
    pub author: String,
    pub quote: String,
    pub tags: Vec<String>,
    pub id: String,
    pub image: String,
    pub length: i64,
}

struct QODContents {
    pub quotes: Vec<Struct>,
}

struct QODRoot {
    pub success: String,
    pub contents: Contents,
}