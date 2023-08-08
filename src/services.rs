use actix_web::{ get, post, web::{ Data, Json, Path }, Responder, HttpResponse };
use serde::{ Deserialize, Serialize };
use sqlx::{ self, FromRow };
use crate::AppState;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
  id: i32,
  first_name: String,
  last_name: String,
}


#[derive(Serialize, Deserialize, FromRow)]
pub struct AddUser {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    content: String,
    created_by: i32,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()

    match
        sqlx
            ::query_as::<_, User>("SELECT id, first_name, last_name FROM users")
            .fetch_all(&state.db).await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No users found"),
    }
}

#[post("/users")]
pub async fn add_users(state: Data<AppState>, new_d: Json<AddUser>) -> impl Responder {
    match
        sqlx
            ::query_as::<_, User>(
                "INSERT INTO users (first_name, last_name) VALUES ($1, $2) RETURNING id, first_name, last_name"
            )
            .bind(new_d.first_name.to_string())
            .bind(new_d.last_name.to_string())
            .fetch_one(&state.db).await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

//
#[post("/users/{id}/articles")]
pub async fn create_user_article(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<CreateArticleBody>
) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("POST /users/{id}/articles")

    match
        sqlx
            ::query_as::<_, Article>(
                "INSERT INTO articles (title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by"
            )
            .bind(body.title.to_string())
            .bind(body.content.to_string())
            .bind(id)
            .fetch_one(&state.db).await
    {
        Ok(article) => HttpResponse::Ok().json(article),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create user article"),
    }
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /users/{id}/articles")

    match
        sqlx
            ::query_as::<_, Article>(
                "SELECT id, title, content, created_by FROM articles WHERE created_by = $1"
            )
            .bind(id)
            .fetch_all(&state.db).await
    {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(_) => HttpResponse::NotFound().json("No articles found"),
    }
}