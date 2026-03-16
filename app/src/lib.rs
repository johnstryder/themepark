use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::components::Router;

pub fn shell(_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.svg" type="image/svg+xml"/>
                <Title text="Themepark - BALS Stack"/>
                <Stylesheet id="leptos" href=format!("/pkg/themepark.css?v={}", env!("CARGO_PKG_VERSION"))/>
                <meta name="description" content="Themepark - BALS Stack: Bevy, Axum, Leptos, SurrealDB"/>
            </head>
            <body>
                <Router>
                    <App/>
                </Router>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Themepark"/>
        <main class="app-container">
            <header class="app-header">
                <h1>"Themepark"</h1>
                <p class="stack-badge">"BALS Stack: Bevy · Axum · Leptos · SurrealDB"</p>
            </header>

            <section class="game-section">
                <h2>"3D Game View"</h2>
                <div class="game-canvas-wrapper">
                    <GameView/>
                </div>
            </section>

            <section class="ui-section">
                <h2>"UI Panel (Leptos)"</h2>
                <InventoryPanel/>
            </section>
        </main>
    }
}

/// Bevy 3D canvas - only compiled for frontend (hydrate)
#[cfg(feature = "bevy-game")]
#[component]
fn GameView() -> impl IntoView {
    use leptos_bevy_canvas::prelude::*;

    view! {
        <BevyCanvas init=init_bevy_game canvas_id="bevy_canvas" />
    }
}

#[cfg(feature = "bevy-game")]
fn init_bevy_game() -> bevy::prelude::App {
    use bevy::prelude::*;

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#bevy_canvas".into()),
                    resolution: (800, 600).into(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
    )
    .add_systems(Startup, setup_scene);

    app
}

#[cfg(feature = "bevy-game")]
fn setup_scene(
    mut commands: bevy::prelude::Commands,
    mut meshes: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::Mesh>>,
    mut materials: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::StandardMaterial>>,
) {
    use bevy::prelude::*;

    let plane_mesh = meshes.add(Plane3d::default().mesh().size(5.0, 5.0));
    let plane_material = materials.add(Color::srgb(0.3, 0.5, 0.3));
    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(plane_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let cube_mesh = meshes.add(Cuboid::default());
    let cube_material = materials.add(Color::srgb(0.8, 0.2, 0.2));
    commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(cube_material),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Multiple lights for better visibility
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            ..default()
        },
        Transform::from_xyz(-3.0, 5.0, 3.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Placeholder when Bevy is not compiled (SSR) - must match BevyCanvas output exactly for hydration
#[cfg(not(feature = "bevy-game"))]
#[component]
fn GameView() -> impl IntoView {
    view! {
        <canvas id="bevy_canvas"></canvas>
    }
}

#[component]
fn InventoryPanel() -> impl IntoView {
    let items = RwSignal::new(vec!["Health Potion", "Mana Crystal", "Iron Sword"]);
    let selected = RwSignal::new(Option::<usize>::None);

    view! {
        <div class="inventory-panel">
            <ul class="inventory-list">
                {move || {
                    items
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(i, name)| {
                            let is_selected = move || selected.get() == Some(i);
                            view! {
                                <li
                                    class=move || {
                                        if is_selected() { "selected" } else { "" }
                                    }
                                    on:click=move |_| {
                                        selected.set(Some(i));
                                    }
                                >
                                    {name}
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
