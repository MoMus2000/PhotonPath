import pygame
from main import create_image_custom

WIDTH = 512
HEIGHT = 256

screen = pygame.display.set_mode((WIDTH, HEIGHT))


def main():
	running = True
	x, y, z = 0, -0.35, -1
	camera = (x, y, z)
	create_image_custom(camera,"/Users/a./Desktop/ray/animated/pizzatime", compress=True)
	while running:
		camera = (x, y, z)
		for event in pygame.event.get():
			if event.type == pygame.QUIT:
				running = False
			if event.type == pygame.KEYDOWN:
				if event.key == pygame.K_LEFT:
					x-=1
					create_image_custom(camera,"/Users/a./Desktop/ray/animated/pizzatime", compress=True)
				if event.key == pygame.K_RIGHT:
					x+=1
					create_image_custom(camera,"/Users/a./Desktop/ray/animated/pizzatime", compress=True)
				if event.key == pygame.K_UP:
					z+=1
					create_image_custom(camera,"/Users/a./Desktop/ray/animated/pizzatime", compress=True)
				if event.key == pygame.K_DOWN:
					z-=1
					create_image_custom(camera,"/Users/a./Desktop/ray/animated/pizzatime", compress=True)

		img=pygame.image.load("/Users/a./Desktop/ray/animated/pizzatime.png")
		screen.blit(img,(0,0))
		pygame.display.flip()


if __name__ == "__main__":
	main()