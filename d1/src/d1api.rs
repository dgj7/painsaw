use crate::d1wc::M2D_CROSSHAIRS;
use engine::game::GameState;
use engine::graphics::storage::g2d::Graph2D;
use engine::graphics::storage::ui::UIManager;
use engine::window::api::mouse::hide::hide_mouse;
use engine::window::api::mouse::show::show_mouse;

pub(super) fn toggle_main_menu(game: &mut GameState, ui: &mut UIManager, g2d: &mut Graph2D) {
    /* flip the main menu state */
    game.menu = !game.menu;

    /* other variables get set based on whether the main menu should be displayed */
    if game.menu {
        show_mouse();
        ui.activate(1);
        g2d.update(M2D_CROSSHAIRS, |m| m.visible = false);
    } else {
        hide_mouse();
        ui.deactivate();
        g2d.update(M2D_CROSSHAIRS, |m| m.visible = true);
    }
}
