export function waitInJSPromise(ms) {
    return new Promise((resolve, _) => {
        setTimeout(() => {
            resolve(undefined);
        }, ms);
    });
}

export function nonAnimationInterval(callback, ms) {
    const controller = new AbortController();
    const {signal} = controller;
    let handle = -1;
    let lastInstant = Date.now();
    handle = setInterval(() => {
        if (signal.aborted) {
            clearInterval(handle);
            return;
        }
        let prevLastInstant = lastInstant;
        lastInstant = Date.now();
        callback(lastInstant - prevLastInstant);
    }, ms);
    return controller;
}

export function animationInterval(callback, ms) {
    const controller = new AbortController();
    const {signal} = controller;

    // Prefer currentTime, as it'll better sync animtions queued in the 
    // same frame, but if it isn't supported, performance.now() is fine.
    const start = document.timeline ? document.timeline.currentTime : performance.now();

    let lastInstant = start;    

    function frame(time) {
      if (signal.aborted) return;
      let prevLastInstant = lastInstant;
      lastInstant = time;
      callback(time - prevLastInstant);
      scheduleFrame(time);
    }
  
    function scheduleFrame(time) {
      const elapsed = time - start;
      const roundedElapsed = Math.round(elapsed / ms) * ms;
      const targetNext = start + roundedElapsed + ms;
      const delay = targetNext - performance.now();
      setTimeout(() => requestAnimationFrame(frame), delay);
    }
  
    scheduleFrame(start);
    return controller;
}

export class JSTicker {
    constructor(forAnimation, ms) {
        this.start = forAnimation
            ? (document.timeline ? document.timeline.currentTime : performance.now())
            : Date.now();
        this.ms = ms;
        this.lastInstant = this.start;
        this._tickMethod = forAnimation ? this._animationTick.bind(this) : this._nonAnimationTick.bind(this);
    }

    tick(callback) {
        this._tickMethod(callback);
    }

    tickInJSPromise() {
        return new Promise((resolve, _) => {
            this.tick(delta => {
                resolve(delta);
            });
        });
    }

    _animationTick(callback) {
        const elapsed = this.lastInstant - this.start;
        const roundedElapsed = Math.round(elapsed / this.ms) * this.ms;
        const targetNext = this.start + roundedElapsed + this.ms;
        const delay = targetNext - performance.now();
        const frame = time => {
            let prevLastInstant = this.lastInstant;
            this.lastInstant = time;
            callback(time - prevLastInstant);
        };
        setTimeout(() => requestAnimationFrame(frame), delay);
    }

    _nonAnimationTick(callback) {
        setTimeout(() => {
            let prevLastInstant = this.lastInstant;
            this.lastInstant = Date.now();
            callback(this.lastInstant - prevLastInstant);
        }, this.ms);
    }
}