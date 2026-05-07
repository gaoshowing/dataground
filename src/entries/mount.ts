import { createApp, type Component } from "vue";
import "../styles.css";

export function mountWindow(rootComponent: Component) {
  createApp(rootComponent).mount("#app");
}
