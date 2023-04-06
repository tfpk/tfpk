import "./css/tailwind/dist/tailwind.css";

import("./pkg").then((module) => {
  module.run_app();
});
