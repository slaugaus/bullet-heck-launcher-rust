// Stupid, but necessary, hack to do multi-window Slint
// https://github.com/slint-ui/slint/issues/784
macro_rules! gen_settings_window { ()=>{
    slint::slint!(
        import { CheckBox, ComboBox, Button } from "std-widgets.slint";

        export component SettingsWindow inherits Window {
            title: "Settings";
            icon: @image-url("ui/assets/icon_32.png");
            default-font-size: 15px;

            // height: 256px;
            // width: 256px;

            callback res-changed(string);
            
            callback checkbox-changed(string, bool);

            callback save();
            callback reset();
            callback cancel();

            VerticalLayout {
                Text {
                    horizontal-alignment: center;
                    text: "! WORK IN PROGRESS !";
                }

                HorizontalLayout {
                    Text {
                        vertical-alignment: center;
                        text: "Window Resolution: ";
                    }
                    ComboBox {
                        model: [
                            "1366 x 768",
                            "wenis"
                        ];
                        selected(value) => {root.res-changed(value);}
                    }
                }

                CheckBox {
                    text: "Show Framerate";
                    toggled => {root.checkbox-changed(self.text, self.checked);}
                }
                
                CheckBox {
                    text: "Skip Launcher";
                    toggled => {root.checkbox-changed(self.text, self.checked);}
                }

                CheckBox {
                    text: "Autofire";
                    toggled => {root.checkbox-changed(self.text, self.checked);}
                }

                HorizontalLayout {
                    CheckBox {
                        text: "Mute Music";
                        toggled => {root.checkbox-changed(self.text, self.checked);}
                    }
                    CheckBox {
                        text: "Mute Sounds";
                        toggled => {root.checkbox-changed(self.text, self.checked);}
                    }
                }

                HorizontalLayout{
                    Button {
                        text: "Save";
                        clicked => {root.save();}
                    }
                    Button {
                        text: "Reset";
                        clicked => {root.reset();}
                    }
                    Button {
                        text: "Cancel";
                        clicked => {root.cancel();}
                    }
                }
            }
        }
    );
}
}

pub(crate) use gen_settings_window;
