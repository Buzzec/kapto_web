const canvas = document.getElementById("main_canvas") as HTMLCanvasElement;
canvas.width = document.documentElement.clientWidth - 5;
canvas.height = document.documentElement.clientHeight;

const c = canvas.getContext("2d");

// c.fillStyle = "rgba(0, 255, 0, 1)"
// c.fillRect(100, 100, 100, 100);
//
// // Line
// c.beginPath();
// c.moveTo(50, 300);
// c.lineTo(300, 100);
// c.lineTo(400, 300);
// c.strokeStyle = "#fa34a3";
// c.stroke();
//
// c.beginPath();
// c.arc(300, 300, 30, 0, Math.PI * 2, false)
// c.strokeStyle = "blue";
// c.stroke();
//
// for (let i = 0; i < 100; i++){
//     const x = Math.random() * canvas.width;
//     const y = Math.random() * canvas.height;
//     c.beginPath();
//     c.arc(x, y, 30, 0, Math.PI * 2, false);
//     c.strokeStyle = 'blue';
//     c.stroke();
// }
// c.beginPath()
// c.strokeStyle = "blue";
// c.arc(200, 200, 30, 0, Math.PI * 2, false);
// c.stroke();

class Vector {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }

    magnitude(): number {
        return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2));
    }

    unit(): Vector {
        return new Vector(this.x / this.magnitude(), this.y / this.magnitude());
    }

    distance(other: Vector): number {

        return Math.sqrt(Math.pow(this.x - other.x, 2) + Math.pow(this.y - other.y, 2));
    }

    dot(other: Vector): number {
        return (this.x * other.x) + (this.y * other.y);
    }

    project_onto(other: Vector): Vector {
        let other_unit = other.unit();
        return other_unit.mul(this.dot(other_unit));
    }

    add(other: Vector): Vector {
        return new Vector(this.x + other.x, this.y + other.y);
    }

    sub(other: Vector): Vector {
        return new Vector(this.x - other.x, this.y - other.y);
    }

    mul(mul: number): Vector {
        return new Vector(this.x * mul, this.y * mul);
    }

    div(div: number): Vector {
        return new Vector(this.x / div, this.y / div);
    }
}

type ColorType = string | CanvasGradient | CanvasPattern

class Circle {
    id: number;
    radius: number;
    location: Vector;
    velocity: Vector;
    outline: ColorType;
    fill: ColorType;

    constructor(id: number, radius: number, outline: ColorType, fill: ColorType) {
        this.id = id;
        this.radius = radius;
        this.location = new Vector(
            Math.random() * (canvas.width - radius * 2) + radius,
            Math.random() * (canvas.height - radius * 2) + radius
        );
        this.velocity = new Vector(
            10 * 2 * (Math.random() - 0.5),
            10 * 2 * (Math.random() - 0.5)
        );
        this.outline = outline;
        this.fill = fill;
    }

    size(): number {
        return Math.PI * Math.pow(this.radius, 2);
    }

    draw(c: CanvasRenderingContext2D) {
        c.beginPath();
        c.arc(this.location.x, this.location.y, this.radius, 0, Math.PI * 2, false);
        c.strokeStyle = this.outline;
        c.fillStyle = this.fill;
        c.stroke();
        c.fill();
    }

    update(gravity: Vector, circles: Array<Circle>, mouse_location: Vector | undefined) {
        if (mouse_location != undefined) {
            // noinspection JSUnusedLocalSymbols
            const mouse_distance = mouse_location.distance(this.location);
        }

        for (const circle of circles) {
            if (circle.id == this.id) {
                continue;
            }
            let distance_between = this.location.distance(circle.location) - this.radius - circle.radius;
            if (distance_between <= 0) {
                // https://en.wikipedia.org/wiki/Elastic_collision#Two-dimensional_collision_with_two_moving_objects
                const v1 = this.velocity;
                const v2 = circle.velocity;
                const x1 = this.location;
                const x2 = circle.location;
                const m1 = this.size();
                const m2 = circle.size();

                const calc = function (v1: Vector, v2: Vector, x1: Vector, x2: Vector, m1: number, m2: number): Vector {
                    return v1.sub(x1.sub(x2).mul((v1.sub(v2).dot(x1.sub(x2)) / Math.pow(x1.sub(x2).magnitude(), 2)) * 2 * m2 / (m1 + m2)));
                }
                this.velocity = calc(v1, v2, x1, x2, m1, m2);
                circle.velocity = calc(v2, v1, x2, x1, m2, m1)

                // De-collide
                // Me to them
                distance_between -= 1;
                const direction = circle.location.sub(this.location)
                this.location = this.location.add(direction.unit().mul(distance_between * m1 / (m1 + m2)));
                circle.location = circle.location.add(direction.unit().mul(-distance_between * m2 / (m1 + m2)));
            }
        }

        if (this.location.x + this.radius >= canvas.width) {
            this.velocity.x = -Math.abs(this.velocity.x);
        }
        if (this.location.x - this.radius <= 0) {
            this.velocity.x = Math.abs(this.velocity.x);
        }
        if (this.location.y + this.radius >= canvas.height) {
            this.velocity.y = -Math.abs(this.velocity.y);
        }
        if (this.location.y - this.radius <= 0) {
            this.velocity.y = Math.abs(this.velocity.y);
        }

        this.location.x += this.velocity.x;
        this.location.y += this.velocity.y;
        this.velocity = this.velocity.add(gravity);

        if (this.velocity.magnitude() > 20) {
            this.velocity = this.velocity.unit().mul(20);
        }
        // if (this.velocity.magnitude() < 0.1){
        //     this.velocity = this.velocity.unit().mul(0.1);
        // }
    }
}


let mouse_location: Vector | undefined;
window.addEventListener("mousemove", function (event) {
    mouse_location = new Vector(event.x, event.y);
})

function random_byte(): number {
    return Math.floor(Math.random() * 256);
}

function random_color(): string {
    return "rgb(" + random_byte() + ", " + random_byte() + ", " + random_byte() + ")";
}

const circles = new Array<Circle>(30);
for (let x = 0; x < circles.length; x++) {
    circles[x] = new Circle(x, Math.random() * 70 + 30, "rgba(0, 0, 0, 0)", random_color());
}
const gravity = new Vector(0, 0)

function animate() {
    requestAnimationFrame(animate)
    c.clearRect(0, 0, canvas.width, canvas.height);
    for (const circle of circles) {
        circle.update(gravity, circles, mouse_location);
        circle.draw(c);
    }
}

animate();


