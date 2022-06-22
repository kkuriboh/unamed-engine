import init, {
	CollisionBody,
	Engine,
	CollisionShape,
	get_collision_between_collider_and_moving_object,
} from './pkg/engine.js'
/*import {
	load_image,
	// configure_game,
	// game_loop,
	// get_deltatime,
} from './helpers.js'*/

const GAME_WIDTH = 800
const GAME_HEIGHT = 600
const GAME_SPEED = 100
const GAME_ACCELERATION = 10

async function main() {
	await init()
	run()
}

main().catch(console.error)

function run() {
	const engine = new Engine(
		GAME_WIDTH,
		GAME_HEIGHT,
		GAME_SPEED,
		GAME_ACCELERATION
	)
	const coll_body = new CollisionBody(CollisionShape.RECT, 100, 100, 0, 0)
	const coll_body2 = new CollisionBody(CollisionShape.RECT, 100, 100, 0, 0)

	engine.create_element('banana', coll_body, 160, 100)
	engine.create_element('abacate', coll_body2, 100, 100)
	engine.get_element('banana')?.draw_collisions(engine)
	engine.get_element('abacate')?.draw_collisions(engine)

	const teste = get_collision_between_collider_and_moving_object(
		'banana',
		'abacate',
		engine
	)
	console.log({ teste })

	// window.requestAnimationFrame(async () => await game_loop(update, 60))

	// load_player(context)
}

/* async function update() {}

function load_player(context: GameContext) {
	const idle_image = load_image('./assets/toaster_bot/idle.png')
	idle_image.onload = () => {
		let frame = 0
		setInterval(() => {
			context.canvas_ctx.drawImage(
				idle_image,
				0,
				0,
				idle_image.width / 4,
				idle_image.height,
				GAME_WIDTH / 2 - idle_image.width / 4 / 2,
				GAME_HEIGHT - idle_image.height,
				idle_image.width / 4,
				idle_image.height
			)
			frame = frame >= 4 ? 0 : frame + 1
		}, 500)
	}
} */
