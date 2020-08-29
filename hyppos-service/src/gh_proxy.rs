use crate::github_types::{Repo, User};
use crate::State;
use actix_session::Session;
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web, HttpResponse,
};

fn get_user_token(session: Session) -> Result<(User, String), actix_web::Error> {
    match (
        session.get::<User>("user")?,
        session.get::<String>("token")?,
    ) {
        (Some(user), Some(token)) => Ok((user, token)),
        _ => Err(ErrorUnauthorized("log in first").into()),
    }
}

fn err500<O, E: std::fmt::Display>(res: Result<O, E>) -> Result<O, actix_web::Error> {
    res.map_err(|err| ErrorInternalServerError(err.to_string()))
}

pub(crate) async fn get_repos(
    session: Session,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let (user, token) = get_user_token(session)?;
    let repos: Vec<_> = err500(
        state
            .github
            .for_token(&token)
            .get_own_user_repos(&user.login)
            .await,
    )?
    .collect();
    HttpResponse::Ok().json(repos).await
}

pub(crate) async fn list_repo_branches(
    repo: web::Path<String>,
    state: web::Data<State>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let (user, token) = get_user_token(session)?;
    let branches = err500(
        state
            .github
            .for_token(&token)
            .list_branches(&Repo::new(user.login, &*repo))
            .await,
    )?;
    HttpResponse::Ok().json(branches).await
}

// TODO: too many requests, mb there is a way to do this faster?
pub(crate) async fn list_repo_contents(
    branch_info: web::Path<(String, String)>,
    state: web::Data<State>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let (repo, branch) = &*branch_info;
    let (user, token) = get_user_token(session)?;
    let gh = state.github.for_token(&token);
    let repo = Repo::new(user.login, repo);
    let branch = err500(gh.get_branch(&repo, branch).await)?;
    let listing = err500(gh.list_directory(&branch).await)?;
    HttpResponse::Ok().json(listing).await
}

pub(crate) async fn list_directory(
    dir_info: web::Path<(String, String)>,
    state: web::Data<State>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let (repo, dir_sha) = &*dir_info;
    let (user, token) = get_user_token(session)?;
    let repo = Repo::new(user.login, repo);
    let listing = err500(
        state
            .github
            .for_token(&token)
            .list_directory_by_hash(&repo, &dir_sha)
            .await,
    )?;
    HttpResponse::Ok().json(listing).await
}

pub(crate) async fn get_file(
    file_info: web::Path<(String, String)>,
    state: web::Data<State>,
    session: Session
) -> Result<HttpResponse, actix_web::Error> {
    let (repo, file_sha) = &*file_info;
    let (user, token) = get_user_token(session)?;
    let repo = Repo::new(user.login, repo);
    let contents = err500(
        state
            .github
            .for_token(&token)
            .get_file_contents_by_hash(&repo, &file_sha)
            .await,
    )?;
    HttpResponse::Ok().body(contents).await
}
