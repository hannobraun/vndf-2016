#include <assert.h>

#include <stb/image.h>

#include "images.h"


GLuint images_load()
{
	int xSize, ySize, numberOfComponents;

	unsigned char *imageData = stbi_load(
		"images/spaceship.png",
		&xSize, &ySize,
		&numberOfComponents,
		0);
	assert(imageData != NULL);
	assert(numberOfComponents == 4);

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
		xSize,
		ySize,
		0,
		GL_RGBA,
		GL_UNSIGNED_BYTE,
		imageData);

	return textureName;
}
