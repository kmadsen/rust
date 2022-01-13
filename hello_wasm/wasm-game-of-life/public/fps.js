/**
 * This is an example class for calculating frames per second.
 */
export default class Fps {
  constructor() {
    this.fpsElement = document.getElementById("fps");
    this.frames = [];
    this.frames.push(performance.now())
  }

  reset() {
    this.frames = [];
    this.frames.push(performance.now());
  }

  render() {
    // Convert the delta time since the last frame render
    // into a measure of frames per second.
    const now = performance.now();
    
    // Save a window of timestamps.
    this.frames.push(now);
    if (this.frames.length > 60) {
      this.frames.shift();
    }
    
    // Calculate the frames per second and update the text view.
    const deltaSeconds = (this.frames.at(-1) - this.frames[0]) / 1000.0; 
    const framesPerSecond = deltaSeconds > 0 ? this.frames.length / deltaSeconds : 60.0; 
    this.fpsElement.textContent = `Frames per second: ${Math.round(framesPerSecond)}`;
  }
}
