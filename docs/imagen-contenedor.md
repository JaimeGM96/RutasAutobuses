# Imagen del contenedor
En este proyecto, se va a investigar cuales son las diferentes imágenes que existen para crear un contenedor con Rust que nos permita ejecutar nuestro código. Para ello, se expondrán una serie de criterios de selección que nos indicarán los requisitos que deben cumplir las imágenes para ver cual se adapta mejor a nuestras necesidades.

## Criterios de selección
Los criterios de selección que se van a tener en cuenta son los siguientes:
- **Estándar**: En primer lugar, debemos conocer cuál es el estándar del lenguaje y estudiar las imágenes que siguen el mismo.
- **Eficiencia**: Necesitamos que la imagen que elijamos sea eficiente ya que, para poder subir cualquier cambio a producción, es necesario que la imagen se construya lo más rápido posible.
- **Mantenimiento**: La imagen que elijamos tiene que recibir mantenimiento con la mayor frecuencia posible. Esto nos asegura que es un software que no va a dejar de recibir actualizaciones a corto plazo y que, por tanto, no va a quedar obsoleto y no va a aumentar nuestra deuda técnica.
- **Seguridad**: Es necesario que, al igual que el resto de herramientas que usemos y nuestro propio software, la imagen que elijamos sea segura.
- **Tamaño**: El tamaño de la imagen es importante ya que, cuanto más grande sea, más tiempo tardará en descargarse y más espacio ocupará.

## Imágenes
Existen diferentes imágenes que nos permiten crear un contenedor con Rust, tanto oficiales y no oficiales. A continuación, se van a exponer las imágenes que se han estudiado para ver cual se adapta mejor a los criterios de selección.

### [Rust-alpine](https://hub.docker.com/_/rust)
Es la imágen oficial de Rust sobre Alpine Linux. Es una imagen muy ligera pero, de entre las opciones oficiales, es la que mayor tamaño tiene. Como podemos ver en docker hub, es la más segura de todas ya que no se han encontrado vulnerabilidades.

### [Rust-slim-bullseye](https://hub.docker.com/_/rust)
Es la imágen oficial de Rust sobre Debian Bullseye. Esta imagen es más ligera que la de Alpine pero al contrario que con la anterior, si se han encontrado vulnerabilidades.

### [Rust-slim-buster](https://hub.docker.com/_/rust)
Es la imágen oficial de Rust sobre Debian Buster. De las imágenes oficiales, es la más ligera. La diferencia con la anterior no es muy significativa en cuanto a tamaño pero si que dispone de un aumento bastante significativo en las vulnerabilidades.

### [cimg/rust](https://hub.docker.com/r/cimg/rust)
Es una imagen no oficial de Rust. Es la más pesada de todas ya que dispone de multitud de librerías que las otras no traen de serie. 

## Elección
La imagen que se va a elegir es la de Alpine. Cumple con todos los criterios de selección y, aunque no sea la más ligera de todas, es mucho más segura que las demás. 