use crate::RepoInfo;
use crate::author::Author;
use crate::collection::Collection;
use crate::font::Font;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_repo_info,
        get_fonts,
        get_font,
        get_collections,
        get_collection,
        get_authors,
        get_author
    ),
    info(title = "Letter repository", license(name = "ARR"))
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    tag = "Repo",
    path = "/",
    responses(
        (status = 200, description = "Get repo info", body = RepoInfo)
    )
)]
pub fn get_repo_info() {}

#[utoipa::path(
    get,
    tag = "Fonts",
    path = "/fonts",
    responses(
        (status = 200, description = "Get all fonts", body = Vec<Font>),
    ),
    params(
        ("query" = String, Query, description = "Search fonts by name"),
        ("author" = String, Query, description = "Get fonts by author id"),
        ("collection" = String, Query, description = "Get fonts by collection id"),
    )
)]
pub fn get_fonts() {}

#[utoipa::path(
    get,
    tag = "Fonts",
    path = "/fonts/{id}",
    responses(
        (status = 200, description = "Get a font", body = Font),
        (status = 404, description = "Font not found"),
    ),
    params(
        ("id" = String, Path, description = "Font id"),
    )
)]
pub fn get_font() {}

#[utoipa::path(
    get,
    tag = "Collections",
    path = "/collections",
    responses(
        (status = 200, description = "Get all collections", body = Vec<Collection>),
    ),
    params(
        ("query" = String, Query, description = "Search collections by name"),
        ("type" = String, Query, description = "Get collections by type"),
    )
)]
pub fn get_collections() {}

#[utoipa::path(
    get,
    tag = "Collections",
    path = "/collections/{id}",
    responses(
        (status = 200, description = "Get collection info", body = Collection),
        (status = 404, description = "Collection not found"),
    ),
    params(
        ("id" = String, Path, description = "Collection id"),
    )
)]
pub fn get_collection() {}

#[utoipa::path(
    get,
    tag = "Authors",
    path = "/authors",
    responses(
        (status = 200, description = "Get all authors", body = Vec<Author>),
    ),
    params(
        ("query" = String, Query, description = "Search authors by name"),
    )
)]
pub fn get_authors() {}

#[utoipa::path(
    get,
    tag = "Authors",
    path = "/authors/{id}",
    responses(
        (status = 200, description = "Get author info", body = Author),
        (status = 404, description = "Author not found"),
    ),
    params(
        ("id" = String, Path, description = "Author id"),
    )
)]
pub fn get_author() {}
