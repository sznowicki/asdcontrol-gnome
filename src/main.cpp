#include <gtkmm.h>

int main(int argc, char* argv[]) {
    auto app = Gtk::Application::create("com.example.asdcontrol");

    Gtk::Window window;
    window.set_default_size(800, 600);
    window.set_title("asdcontrol-gnome");

    return app->make_window_and_run<Gtk::Window>(argc, argv);
}
