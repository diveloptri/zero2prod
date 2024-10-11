use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;
use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"))
    };

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(
            r#"
            <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Change password</title>
</head>
<body>
    {msg_html}
    <form action="/admin/password" method="post">
        <label for="cpassword">Current password
            <input type="password" placeholder="Enter current password" name="cpassword">
        </label>
        <br>
        <label for="npassword">New password
            <input type="password" placeholder="Enter new password" name="npassword">
        </label>
        <br>
        <label for="new_password_check">Confirm new password
            <input type="password" placeholder="Type the new password agian" name="new_password_check">
        </label>
        <br>
        <button type="submit">Change password</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#
    ))
}