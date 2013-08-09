#include <assert.h>

#include <stb/image.h>

#include "images.h"


image  loadImage(void);
GLuint createTexture(image image);


GLuint images_load()
{
	image  image       = loadImage();
	GLuint textureName = createTexture(image);

	return textureName;
}

image loadImage()
{
	image image;
	int numberOfComponents;

	image.data = stbi_load(
		"images/spaceship.png",
		&image.width, &image.height,
		&numberOfComponents,
		0);

	assert(image.data != NULL);
	assert(numberOfComponents == 4);

	return image;
}

GLuint createTexture(image image)
{
	// Generate texture names.
	GLuint textureName;
	glGenTextures(1, &textureName);

	glBindTexture(
		GL_TEXTURE_2D,
		textureName);

	// Configure texture.
	glTexParameteri(
		GL_TEXTURE_2D,
		GL_TEXTURE_MIN_FILTER,
		GL_NEAREST);

	// Bind image data to texture name.
	glTexImage2D(
		GL_TEXTURE_2D,
		0,
		GL_RGBA8,
		image.width,
		image.height,
		0,
		GL_RGBA,
		GL_UNSIGNED_BYTE,
		image.data);

	return textureName;
}
