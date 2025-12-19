import * as THREE from 'three';
import { OrbitControls } from 'orbitcontrols';

let scene, camera, renderer, controls;
let robot;

const moveSpeed = 0.1;
const turnSpeed = 0.05;
const keys = {};

const container = document.getElementById('minirobot-viewport');

init();
animate();

function init() {
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x202020);

  camera = new THREE.PerspectiveCamera(60, 1, 0.1, 100);
  camera.position.set(3, 2, 4);

  renderer = new THREE.WebGLRenderer({ antialias: true });
  container.appendChild(renderer.domElement);

  resizeRendererToContainer();

  controls = new OrbitControls(camera, renderer.domElement);

  scene.add(new THREE.AmbientLight(0xffffff, 0.6));

  const dirLight = new THREE.DirectionalLight(0xffffff, 1);
  dirLight.position.set(5, 10, 5);
  scene.add(dirLight);

  createRobot();

  window.addEventListener('resize', resizeRendererToContainer);
  window.addEventListener('keydown', e => keys[e.key.toLowerCase()] = true);
  window.addEventListener('keyup', e => keys[e.key.toLowerCase()] = false);
}

function createRobot() {
  robot = new THREE.Group();

  const body = new THREE.Mesh(
    new THREE.BoxGeometry(1, 0.4, 0.6),
    new THREE.MeshStandardMaterial({ color: 0x00ff88 })
  );
  body.position.y = 0.3;
  robot.add(body);

  const wheelGeo = new THREE.CylinderGeometry(0.15, 0.15, 0.4, 16);
  const wheelMat = new THREE.MeshStandardMaterial({ color: 0x333333 });

  [-1, 1].forEach(side => {
    const wheel = new THREE.Mesh(wheelGeo, wheelMat);
    wheel.rotation.z = Math.PI / 2;
    wheel.position.set(0.4 * side, 0.15, 0.35);
    robot.add(wheel);
  });

  scene.add(robot);
}

function animate() {
  requestAnimationFrame(animate);
  updateRobot();
  renderer.render(scene, camera);
}

function updateRobot() {
  if (keys['w']) robot.translateZ(-moveSpeed);
  if (keys['s']) robot.translateZ(moveSpeed);
  if (keys['a']) robot.rotation.y += turnSpeed;
  if (keys['d']) robot.rotation.y -= turnSpeed;
}

function resizeRendererToContainer() {
  const width = container.clientWidth;
  const height = container.clientHeight;

  renderer.setSize(width, height, false);
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
}
