#include "images.h"

#include <assert.h>

#include <stb/image.h>


image  loadImage(void);
GLuint createTexture(image image);


Texture images_load()
{
	image  img         = loadImage();
	GLuint textureName = createTexture(img);

	Texture texture = {textureName, img.width, img.height};

	return texture;
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
