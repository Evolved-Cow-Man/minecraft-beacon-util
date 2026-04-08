/*
TODO

1. make some way to build it in minecraft
*/

// wasm
import init, {
  generate_positions,
  generate_colors,
} from "./pkg/minecraft_beacon.js";
await init();

//console.log(color_accuracy_data)

const colorAccuracyLabel = document.getElementById("colorAccuracyLabel");

function customEventListener(id, is_checkbox, is_url) {
  const input = document.getElementById(id);

  // initialize settings
  if (is_checkbox) {
    settings[id] = Number(input.checked);
  } else {
    settings[id] = Number(input.value);
  }

  // initialize label
  const label = document.getElementById(id + "Label");
  if (is_checkbox) {
    if (settings[id]) {
      label.innerHTML = "yes";
    } else {
      label.innerHTML = "no";
    }
  } else {
    label.innerHTML = settings[id];
  }

  // read URL params
  if (is_url) {
    const url_params = new URLSearchParams(window.location.search);
    if (url_params.has(id)) {
      if (is_checkbox) {
        input.checked = Number(url_params.get(id));
      } else {
        input.value = Number(url_params.get(id));
      }
    }
  }

  input.addEventListener("input", function () {
    // update settings
    if (is_checkbox) {
      settings[id] = Number(input.checked);
    } else {
      settings[id] = Number(input.value);
    }
    console.log("input", id, settings[id]);

    // update label
    if (is_checkbox) {
      if (settings[id]) {
        label.innerHTML = "yes";
      } else {
        label.innerHTML = "no";
      }
    } else {
      label.innerHTML = settings[id];
    }

    // write URL params
    if (is_url) {
      const url = new URL(window.location);
      if (settings[id] == input.defaultValue) {
        url.searchParams.delete(id);
        window.history.replaceState({}, "", url);
      } else {
        url.searchParams.set(id, settings[id]);
        window.history.replaceState(null, "", url);
      }
    }

    updateApp();
  });

  if (!is_checkbox) {
    input.addEventListener("dblclick", function () {
      console.log("dblclick", id);
      input.value = input.defaultValue;
      input.dispatchEvent(new Event("input")); // trigger change
    });
  }
}

const settings = {};
customEventListener("displaySize", false, false);
customEventListener("numberOfBeacons", false, true);
customEventListener("minRadius", false, true);
customEventListener("maxRadius", false, true);
customEventListener("rotationOffset", false, true);
customEventListener("lightness", false, true);
customEventListener("chroma", false, true);
customEventListener("hueOffset", false, true);
customEventListener("clockwiseHue", true, true);

const app = document.getElementById("app");
updateApp();

function updateApp() {
  const positions = generate_positions(
    settings.numberOfBeacons,
    settings.minRadius,
    settings.maxRadius,
    settings.rotationOffset,
  );
  // offset hue so colors stay positionally the same
  const colors = generate_colors(
    settings.numberOfBeacons,
    settings.lightness,
    settings.chroma,
    settings.hueOffset - settings.rotationOffset,
    settings.clockwiseHue,
  );
  //console.log(colors)
  let last_color_position = 0;

  // take the absolute value for each value in the original array and find the max
  const offset = Math.max(...positions.map(Math.abs));

  app.innerHTML = "";
  for (let beacon_id = 0; beacon_id < settings.numberOfBeacons; beacon_id++) {
    const new_div = document.createElement("div");
    new_div.classList.add("beacon");
    new_div.style.width = settings.displaySize + "px";
    new_div.style.height = settings.displaySize + "px";

    const x_original = positions[beacon_id * 2];
    let x = x_original + offset;
    x *= settings.displaySize;
    new_div.style.left = x + "px"; // distance from left of container

    const y_original = positions[beacon_id * 2 + 1];
    let y = y_original * -1; // large y values should appear near the top of the screen
    y += offset;
    y *= settings.displaySize;
    new_div.style.top = y + "px"; // distance from top of container

    // tooltip
    const tooltip = document.createElement("div");
    tooltip.classList.add("tooltip");
    tooltip.style.left = settings.displaySize + "px";
    new_div.append(tooltip);

    for (let i = last_color_position; i < colors.length; i++) {
      // if the first value in the string is a color
      if (colors[i][0] == "#") {
        new_div.style.backgroundColor = colors[i];

        for (let j = i + 1; j < colors.length; j++) {
          if (colors[j][0] != "#") {
            // tooltip inside
            const tooltip_container = document.createElement("div");
            tooltip_container.style.display = "flex";
            tooltip_container.style.alignItems = "center";

            const tooltip_glass = document.createElement("img");
            tooltip_glass.style.height = "1em";
            tooltip_glass.style.width = "auto";
            tooltip_glass.src = "./glass/" + colors[j] + ".png";
            tooltip_container.append(tooltip_glass);

            const tooltip_text = document.createElement("span");
            tooltip_text.innerHTML =
              "&nbsp;" +
              colors[j].replace("_stained_glass", "").replace("_", "&nbsp;");
            tooltip_container.append(tooltip_text);

            tooltip.prepend(tooltip_container);
          } else {
            last_color_position = j;
            break;
          }
        }
        // position display
        const position_display = document.createElement("span");
        position_display.innerHTML =
          "Position:<br>(" + x_original + ",&nbsp;" + y_original + ")";
        tooltip.prepend(position_display);

        break;
      }
    }
    app.append(new_div);
  }

  colorAccuracyLabel.innerHTML =
    color_accuracy_data[settings.lightness][settings.chroma];
}
