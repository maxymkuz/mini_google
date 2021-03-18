mod lib;

/// Example usage of the query sender
#[tokio::main]
async fn main() {
    lib::send_lang_detection_query(
        "Sangre que no se desborda, juventud que no se atreve,
        ni es sangre, ni es juventud, ni relucen, ni florecen. Cuerpos que nacen vencidos,
        vencidos y grises mueren: vienen con la edad de un siglo,
        y son viejos cuando vienen",
    )
    .await
    .unwrap();
}
