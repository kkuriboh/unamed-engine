export function configure_game(
	canvas: HTMLCanvasElement,
	game_width: number,
	game_height: number
): CanvasRenderingContext2D {
	canvas.width = game_width
	canvas.height = game_height
	canvas.style.backgroundColor = '#8888ff'
	const ctx = canvas.getContext('2d')!
	ctx.font = '1rem Kdam Thmor Pro'
	return ctx
}

export function load_image(path: string): HTMLImageElement {
	const image = new Image()

	if (path === '' || path === undefined)
		image.src = 'https://source.unsplash.com/random'
	else image.src = path

	return image
}

export async function game_loop(update: () => Promise<void>, frame_rate: number) {
	await update()

	await new Promise((res) => setTimeout(res, frame_rate * get_deltatime()))

	window.requestAnimationFrame(async () => await game_loop(update, frame_rate))
}

export function get_deltatime(): number {
	return window.performance.now() / 1000
}
