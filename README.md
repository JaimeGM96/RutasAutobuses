# Proyecto IV

## Descripción del problema 
Los usuarios que quieran moverse entre municipios con transporte público tienen diferentes opciones. Una de esas opciones es el autobús. En la actualidad, se dispone de poca información para realizar un trayecto en una línea de autobús de la Red de Consorcios de Transporte de Andalucía ya que dicha información se encuentra en las diferentes marquesinas de las paradas y, como se ha comentado, es escasa.

El problema que tienen los usuarios del transporte en autobús entre municipios es la falta de información de las posibles paradas que se pueden realizar en el trayecto con los posibles transbordos que se pueden efectuar en el recorrido.

Dados los datos de los trayectos de las líneas de autobús de la [Red de Consorcios de Transporte de Andalucía](https://api.ctan.es/doc/#api-Corredores-ObtieneBloquesCorredor), podremos paliar este problema procesando la información poco estructurada de la que disponemos y ofreciendo cálculos de rutas entre municipios con los transbordos necesarios.

## Elección de herramientas
Para la elección del gestor de dependencias y del gestor de tareas, se han tenido en cuenta una serie de criterios que se pueden consultar en la documentación del proyecto. La elección de los mismos ha sido, en gran medida, por ser el estándar del lenguaje pero el resto de criterios también se han tenido en cuenta.

## Descripción de la solución
Para alcanzar la solución, se hace uso de la clase BuscadorRutas que es la encargada de albergar la lógica de negocio.

Para comprobar que la sintaxis es correcta, se puede ejecutar el siguiente comando:
```bash
cargo check
```

## Ejecución de las pruebas
Para comprobar que el código funciona correctamente, se puede ejecutar el siguiente comando:
```bash
cargo test
```

## Contenedor de pruebas
Para construir el contenedor, se puede ejecutar el siguiente comando:
```bash
docker build --no-cache -t jaimegm96/rutasautobuses .
```

Para ejecutar el contenedor, se puede ejecutar el siguiente comando:
```bash
docker run -tv `pwd`:/app/test jaimegm96/rutasautobuses
```

## Documentación
- [Configuración de git](docs/configuracion-git.md).
- [Usuarios de la aplicación](docs/usuarios.md).
- [Historias de usuario](docs/historias-usuarios.md).
- [Milestones](docs/milestones.md).
- [Gestor de dependencias](docs/gestor-dependencias.md).
- [Gestor de tareas](docs/gestor-tareas.md).
- [Herramientas de testing](docs/herramienta-testing.md).
- [Imagen del contenedor](docs/imagen-contenedor.md).