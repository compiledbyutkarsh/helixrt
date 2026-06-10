document.querySelector("#app").innerHTML = `
  <div class="container">
    <div class="title">
      HelixRT Runtime Dashboard
    </div>

    <div class="panel">
      <div class="metric" id="position">
        Position:
      </div>

      <div class="metric" id="occupancy">
        Occupancy:
      </div>

      <div class="metric" id="heading">
        Heading:
      </div>
    </div>

    <canvas
      id="map"
      width="900"
      height="500"
    ></canvas>
  </div>
`;

const style =
  document.createElement("style");

style.innerHTML = `
  body {
    margin: 0;
    background: #0d1117;
    color: white;
    font-family: Inter, Arial, sans-serif;
  }

  .container {
    padding: 32px;
  }

  .title {
    font-size: 34px;
    font-weight: 700;
    margin-bottom: 28px;
  }

  .panel {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 14px;
    padding: 24px;
    margin-bottom: 24px;
  }

  .metric {
    font-size: 18px;
    margin: 10px 0;
  }

  canvas {
    background: #161b22;
    border-radius: 14px;
    border: 1px solid #30363d;
  }
`;

document.head.appendChild(style);

const positionElement =
  document.getElementById(
    "position"
  );

const occupancyElement =
  document.getElementById(
    "occupancy"
  );

const headingElement =
  document.getElementById(
    "heading"
  );

const canvas =
  document.getElementById("map");

const context =
  canvas.getContext("2d");

const socket =
  new WebSocket(
    "ws://127.0.0.1:9001"
  );

let robotX = 450;
let robotY = 250;

const trail = [];

socket.onmessage = (event) => {
  try {
    const packet =
      JSON.parse(event.data);

    positionElement.innerText =
      `Position: (${packet.x.toFixed(2)}, ${packet.y.toFixed(2)})`;

    occupancyElement.innerText =
      `Occupancy: ${(packet.occupancy * 100).toFixed(2)}%`;

    headingElement.innerText =
      `Heading: ${packet.heading.toFixed(2)}`;

    robotX =
      450 + packet.x * 4;

    robotY =
      250 + packet.y * 4;

    trail.push({
      x: robotX,
      y: robotY,
    });

    if (trail.length > 120) {
      trail.shift();
    }
  } catch {}
};

function renderGrid() {
  context.strokeStyle =
    "#222831";

  for (
    let x = 0;
    x < canvas.width;
    x += 40
  ) {
    context.beginPath();

    context.moveTo(x, 0);

    context.lineTo(
      x,
      canvas.height
    );

    context.stroke();
  }

  for (
    let y = 0;
    y < canvas.height;
    y += 40
  ) {
    context.beginPath();

    context.moveTo(0, y);

    context.lineTo(
      canvas.width,
      y
    );

    context.stroke();
  }
}

function renderTrail() {
  context.fillStyle =
    "#58a6ff";

  for (const point of trail) {
    context.beginPath();

    context.arc(
      point.x,
      point.y,
      2,
      0,
      Math.PI * 2
    );

    context.fill();
  }
}

function renderRobot() {
  context.fillStyle =
    "#2ea043";

  context.beginPath();

  context.arc(
    robotX,
    robotY,
    10,
    0,
    Math.PI * 2
  );

  context.fill();
}

function render() {
  context.clearRect(
    0,
    0,
    canvas.width,
    canvas.height
  );

  renderGrid();

  renderTrail();

  renderRobot();

  requestAnimationFrame(
    render
  );
}

render();