mod invoice;
mod recipient;
mod server;

use invoice::Invoice;

#[tokio::main]
async fn main() -> Result<(), invoice::Error> {
    let _unused = recipient::decode_recipient();

    let invoice = Invoice::with_amount("timebrand03@walletofsatoshi.com", 1_000).await?;
    invoice.print_qr_code();
    invoice.save_qr_code();

    let addr = "0.0.0.0:3987";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening at http://{addr}");
    axum::serve(listener, server::routes()).await.unwrap();

    Ok(())
}
