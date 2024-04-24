use fedimint_core::core::ModuleKind;
use fedimint_core::fedimint_build_code_version_env;
use fedimint_dummy_common::config::DummyGenParams;
use fedimint_dummy_server::DummyInit;
use fedimintd::Fedimintd;

const KIND: ModuleKind = ModuleKind::from_static_str("dummy");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Fedimintd::new(fedimint_build_code_version_env!())?
        .with_default_modules()
        .with_module_kind(DummyInit)
        .with_module_instance(KIND, DummyGenParams::default())
        .run()
        .await
}
