# El Secreto de Zaculeu

El Secreto de Zaculeu es un ray caster desarrollado en Rust con Raylib. El juego está inspirado en las ruinas mayas de Zaculeu, en Huehuetenango, Guatemala.

¿Incluye dos niveleles completamente jugables, colisiones con paredes, cámara en primera persona, sprites, texturas, sonido y soporte para control.

## Video de demostración

https://youtu.be/kqXUokJaS5Y?si=aU1LwcnQa07pvpDe

## Objetivo del juego

El jugador debe explorar las ruinas mientras Kukulkán lo persigue. En cada nivel se encuentran tres piedras con números mayas y un artefacto ceremonial. Las pistas revelan un código de tres dígitos que debe ingresarse en el mural de la salida.

Kukulkán puede evitarse o enfrentarse con lanzamientos de lanza. Si alcanza al jugador, la partida termina. Un código incorrecto aumenta su velocidad y hace más difícil llegar a la salida.

## Niveles

El menú de bienvenida permite elegir entre dos niveles:

1. Plaza de Zaculeu: utiliza suelo gris, estelas y muros de piedra para representar el área de las ruinas.
2. Patio ceremonial: utiliza suelo verde, vegetación y una distribución diferente de paredes.

Cada mapa incluye paredes con diferentes texturas, puertas, pistas, un artefacto, el mural de salida y una posición inicial para Kukulkán.

## Controles de teclado y mouse

| Acción | Control |
| --- | --- |
| Avanzar y retroceder | `W`, `S` o flechas arriba y abajo |
| Girar la cámara | Movimiento horizontal del mouse |
| Rotación alternativa | `A`, `D` o flechas izquierda y derecha |
| Tirar la lanza | `Espacio` |
| Alternar entre vista 3D y mapa 2D | `M` |
| Volver al menú de inicio | `R` |
| Alternar pantalla completa | `F11` |
| Cerrar el juego | `Esc` |

En el menú se usan `W`, `S` o las flechas para elegir un nivel. `Enter` inicia la partida y `M` activa o silencia la música.

En el mural se escriben los tres dígitos con las teclas numéricas. `Backspace` borra y `Enter` confirma el código.

## Controles con mando

| Acción | Control |
| --- | --- |
| Avanzar y retroceder | Joystick izquierdo |
| Girar la cámara | Joystick derecho |
| Lanzar la lanza | `R2` |
| Alternar entre vista 3D y mapa 2D | `Triángulo` |
| Volver al menú de inicio | `Options` |

En el menú, el joystick izquierdo selecciona el nivel, `X` confirma y `Cuadrado` activa o silencia la música.

En el mural, el joystick izquierdo selecciona una posición y cambia su valor. `X` confirma el código y `Círculo` borra los dígitos ingresados.

## Requisitos

- Rust estable con Cargo.
- Un compilador de C y C++ compatible con el sistema.
- CMake si la compilación de Raylib lo solicita.
- Windows, Linux o macOS con soporte para OpenGL.

El soporte de mando fue desarrollado para DualShock 4. Otros controles reconocidos por Raylib pueden funcionar con una distribución equivalente.

## Instalación y ejecución

Clona el repositorio:

```bash
git clone https://github.com/josesan28/proyecto-ray-caster.git
cd proyecto-ray-caster
```

Compila y ejecuta el juego desde la raíz del proyecto:

```bash
cargo run --release
```

Es importante ejecutar el comando desde la raíz porque las texturas, niveles y archivos de audio se cargan mediante rutas relativas dentro de `assets`.

Para comprobar que el proyecto compila sin iniciarlo:

```bash
cargo check
```

## Funciones implementadas

| Criterio | Implementación |
| --- | --- |
| Nivel entero y jugable | Dos mapas con inicio, objetivo, salida y condición de derrota |
| Colisiones | El jugador y Kukulkán respetan los límites de las paredes |
| Paredes diferenciadas | Texturas distintas para piedra, vegetación, estelas y puertas |
| Rendimiento | Ciclo gráfico configurado con contador visible |
| Cámara con movimiento | Avance y retroceso con teclado, rotación horizontal con mouse |
| Soporte para control | Movimiento, rotación, disparo, mapa, menú y mural con DualShock 4 |
| Disparo | Lanzamiento de lanza con animación, sonido y tiempo de recarga |
| Minimapa | Muestra al jugador, su dirección y a Kukulkán |
| Música de fondo | Música en bucle con opción para activarla o silenciarla |
| Efectos de sonido | Sonidos para la lanza, las pistas, el artefacto y Kukulkán |
| Animaciones | Kukulkán, el artefacto y el lanzamiento de la lanza tienen animación |
| Pantalla de bienvenida | Menú inicial con instrucciones y selección de dos niveles |
| Pantalla de éxito | Se muestra al ingresar el código correcto y abrir la salida |
| Pantalla de derrota | Se muestra cuando Kukulkán alcanza al jugador |
| Pantalla completa | `F11` alterna entre ventana y pantalla completa sin deformar la imagen |

## Implementación técnica

El escenario se almacena como una matriz de caracteres. Cada símbolo representa una pared, una textura, una puerta o un elemento interactivo.

El renderizado 3D lanza un rayo por cada columna de la pantalla. La distancia corregida evita la distorsión de ojo de pez y determina la altura visible de cada pared. Un búfer de profundidad permite ocultar correctamente los sprites cuando hay una pared delante de ellos.

El movimiento utiliza el tiempo transcurrido entre cuadros para mantener una velocidad consistente. La colisión revisa varios puntos alrededor del jugador antes de aceptar una nueva posición, por lo que no es posible atravesar las paredes.

La resolución virtual es de `800 × 600`. Al activar pantalla completa, todo el contenido se escala conservando la proporción original.

## Estructura del proyecto

```text
src/
  main.rs         Flujo principal y ciclo del juego
  caster.rs       Lanzamiento de rayos y detección de impactos
  renderer.rs     Renderizado 3D, sprites y minimapa
  controller.rs   Entrada de teclado, mouse y control
  game.rs         Estado, pistas, mural y condiciones del nivel
  combat.rs       Detección de disparos contra Kukulkán
  sprite.rs       Jugador, enemigo, artefacto y pistas
  textures.rs     Carga y generación de texturas
  hud.rs          Interfaz, mensajes y pantallas finales
  menu.rs         Pantalla de bienvenida y selección de nivel
  display.rs      Pantalla completa y escalado de la imagen
assets/
  audio/          Música y efectos de sonido
  levels/         Mapas de los niveles
  textures/       Texturas y sprites
```
