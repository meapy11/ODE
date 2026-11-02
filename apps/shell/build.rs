use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("org.ode.shell")
            .qml_file("qml/Main.qml"),
    )
    .build();
}