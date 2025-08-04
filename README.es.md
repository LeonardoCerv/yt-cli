![yt-cli](/logo.png)

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![yt-dlp](https://img.shields.io/badge/yt--dlp-FF0000?logo=youtube&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-brown)

Descarga videos o listas de reproducción de YouTube a tu computadora. Funciona también con **videos privados y no listados** si le das tus cookies del navegador.

## ¿Primera vez usando la línea de comandos? ¡Lee esto!

Si nunca has usado una "terminal" o "línea de comandos", ¡no te preocupes! Esto es lo que necesitas saber:

### ¿Qué es una Terminal/Línea de Comandos?

Piénsalo como una forma de hablar con tu computadora usando texto. En vez de hacer clic con el ratón, escribes comandos y presionas Enter. Suena complicado, pero en realidad es sencillo.

**¿Qué es "bash"?**  
Bash es el lenguaje que tu computadora usa para entender los comandos que escribes. Cuando veas bloques de código que empiezan con ```bash, esos son comandos que debes escribir.

### Cómo abrir la Terminal:

**En Mac:**
1. Presiona Cmd+Espacio (abre Spotlight)
2. Escribe "terminal"
3. Presiona Enter
4. Se abrirá una ventana negra o blanca: ¡esa es tu terminal!

**En Windows:**
1. Presiona la tecla Windows + R
2. Escribe "cmd" y presiona Enter
3. Se abrirá una ventana negra: es el símbolo del sistema (igual que la terminal)

**Alternativa en Windows:**
1. Haz clic en el botón de Inicio
2. Escribe "símbolo del sistema"
3. Haz clic en "Símbolo del sistema" cuando aparezca

### Cómo usar los comandos:

1. **¿Ves un comando en esta guía?** Como `yt-cli --help`
2. **Copia todo** (sin el ```bash)
3. **Pégalo en tu terminal** (clic derecho → pegar, o Ctrl+V en Windows, Cmd+V en Mac)
4. **Presiona Enter**
5. **Espera a que termine** - verás texto aparecer, y cuando termine, verás una nueva línea con el cursor esperando el siguiente comando

**Importante:**  
- No cierres la ventana de la terminal mientras los comandos estén corriendo
- Si algo se traba, normalmente puedes detenerlo con Ctrl+C
- Si ves un "%" o "$" al inicio de una línea, solo indica dónde escribir - no escribas esos símbolos

### Explicación de rutas de archivos:

Cuando veas `~/Desktop/cookies.txt`, significa:
- `~` = tu carpeta personal (como /Users/TuNombre en Mac, o C:\Users\TuNombre en Windows)
- `/` = separa carpetas (como hacer clic en carpetas)
- Así que `~/Desktop/cookies.txt` es "el archivo cookies.txt en tu Escritorio"

**Usuarios de Windows:** Tus rutas son diferentes - usa `C:\Users\TuNombre\Desktop\cookies.txt`

¡Ahora sí, empecemos!

## Instalación (Paso a Paso)

### 1. Instala yt-dlp primero

**En Mac:**
1. Abre la app Terminal (sigue las instrucciones arriba si no sabes cómo)
2. Verás una ventana con texto y un cursor. Aquí escribes los comandos.
3. Copia este comando: `brew install yt-dlp`
4. Pégalo en la terminal (clic derecho → pegar, o Cmd+V)
5. Presiona Enter y espera
    - Si dice "brew command not found", necesitas Homebrew primero:
    - Copia esta línea: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
    - Pégala en la terminal, presiona Enter y espera (tarda 5-10 minutos)
    - Luego intenta de nuevo `brew install yt-dlp`

**En Windows:**
1. Abre el Símbolo del sistema (sigue las instrucciones arriba)
2. Primero, asegúrate de tener Python instalado:
    - Ve a python.org en tu navegador
    - Descarga Python (el gran botón amarillo)
    - Ejecuta el instalador y marca "Add Python to PATH"
3. De vuelta en el símbolo del sistema, copia esto: `pip install yt-dlp`
4. Pega y presiona Enter
5. Espera a que termine

**En Linux:**
1. Abre la Terminal
2. Copia esto: `pip install yt-dlp`
3. Pega y presiona Enter
4. Si pip no está instalado, prueba: `sudo apt install python3-pip` (en Ubuntu/Debian)

### 2. Instala Rust (necesario para compilar esta herramienta)

**Todos deben hacer esto:**

1. Ve a https://rustup.rs en tu navegador
2. Verás un comando grande que empieza con `curl` - cópialo completo
3. Abre la Terminal (Mac/Linux) o Símbolo del sistema (Windows)
4. Pega el comando y presiona Enter
5. Verás texto desplazándose - es normal, solo espera
6. Cuando pregunte "Proceed with installation (y/n)?", escribe `y` y presiona Enter
7. Espera a que termine (tarda 5-10 minutos - verás "Rust is installed now" al final)
8. **Importante:** Cierra la terminal completamente y abre una nueva
9. Prueba que funcionó: escribe `cargo --version` y presiona Enter
10. Si ves un número de versión como "cargo 1.70.0", ¡todo bien! Si no, repite el paso 8.

### 3. Descarga e instala esta herramienta

**Opción A: Descargar desde GitHub (más fácil)**
1. Ve a la página de GitHub de este proyecto
2. Haz clic en el botón verde "Code"
3. Haz clic en "Download ZIP"
4. Busca el archivo ZIP en tu carpeta de Descargas y descomprímelo
5. Obtendrás una carpeta llamada algo como "yt-cli-main" o "media-saver-main"

**Opción B: Usa git (si sabes lo que es)**
```bash
git clone https://github.com/LeonardoCerv/media-saver.git
```

**Ahora instálalo:**
1. Abre la Terminal/Símbolo del sistema (¡ya sabes cómo!)
2. Navega a la carpeta que descargaste. Así:
    - **Fácil:** Escribe `cd ` (c y d, seguido de un espacio)
    - Luego arrastra la carpeta desde el explorador de archivos (Finder en Mac, Explorador en Windows) a la terminal
    - Presiona Enter
    - **Manual:** Escribe algo como `cd ~/Downloads/yt-cli-main` (pero cambia la ruta según donde esté tu carpeta)
3. Ahora deberías estar "dentro" de esa carpeta en la terminal
4. Copia este comando: `cargo install --path .`
5. Pega y presiona Enter
6. **Espera pacientemente** - puede tardar 5-15 minutos según tu computadora
7. Verás mucho texto diciendo "Compiling..." - ¡es normal!
8. Cuando termine, verás algo como "Installed package `yt-cli v0.3.2`"
9. Si ves texto rojo de error, revisa la sección de solución de problemas

### 4. Prueba que funciona

1. **Cierra la terminal completamente** y abre una nueva (¡esto es importante!)
2. Escribe este comando: `yt-cli --help`
3. Presiona Enter
4. Si ves un texto de ayuda con opciones, ¡felicidades! ¡Funciona!
5. Si dice "command not found" o similar:
    - Asegúrate de cerrar la terminal vieja y abrir una nueva
    - Prueba esto: `~/.cargo/bin/yt-cli --help`
    - Si eso funciona, necesitas agregar ~/.cargo/bin a tu PATH (busca en Google "agregar cargo bin a PATH" + tu sistema operativo)

**Prueba final - descarga algo:**
1. Asegúrate de estar en una carpeta donde quieras guardar los videos (como tu Escritorio)
2. Para ir al Escritorio en la terminal: `cd ~/Desktop` (Mac/Linux) o `cd %USERPROFILE%\Desktop` (Windows)
3. Copia este comando: `yt-cli "https://youtube.com/watch?v=dQw4w9WgXcQ"`
4. Pega y presiona Enter
5. ¡Debería empezar a descargar! Creará una carpeta "downloads" y guardará el video ahí.
6. Si esto funciona, ¡ya está todo listo!

## Uso

### Ejemplos básicos

**Recuerda:** Estos son comandos para escribir en la terminal. Copia cada línea, pégala en la terminal y presiona Enter.

**Descargar cualquier video público:**
```bash
yt-cli "https://youtube.com/watch?v=dQw4w9WgXcQ"
```
(Esto descarga en una carpeta "downloads" en el directorio actual de la terminal)

**Descargar solo el audio como mp3:**
```bash
yt-cli -a "https://youtube.com/watch?v=dQw4w9WgXcQ"
```

**Descargar en una calidad específica (ejemplo 720p):**
```bash
yt-cli -q 720p "https://youtube.com/watch?v=dQw4w9WgXcQ"
```

**Descargar una lista de reproducción completa:**
```bash
yt-cli -p "https://youtube.com/playlist?list=PLrAXtmRdnEQy5VqrkOEE-1T6GrJhgv79C"
```

**Guardar en una carpeta específica (como tu Escritorio):**
```bash
yt-cli -o ~/Desktop/mis-videos "https://youtube.com/watch?v=dQw4w9WgXcQ"
```
(Esto crea una carpeta "mis-videos" en tu Escritorio y guarda ahí)

**Usuarios de Windows:** Para el último ejemplo, usa este formato:
```bash
yt-cli -o C:\Users\TuUsuario\Desktop\mis-videos "https://youtube.com/watch?v=dQw4w9WgXcQ"
```
(Reemplaza "TuUsuario" por tu nombre de usuario real de Windows)

### Para videos privados/no listados

**Paso 1: Obtén tus cookies**

1. **Usuarios de Chrome/Edge:**
    - Abre Chrome o Edge
    - Ve a YouTube.com y asegúrate de estar conectado a tu cuenta
    - Presiona F12 (o clic derecho → "Inspeccionar")
    - Se abrirán las herramientas de desarrollador
    - Arriba, busca pestañas como "Elements", "Console", "Sources", etc.
    - Haz clic en "Application" (si no la ves, haz clic en >> para más pestañas)
    - A la izquierda, busca la sección "Storage"
    - Bajo "Storage", haz clic en "Cookies"
    - Haz clic en "https://www.youtube.com" (elige la principal si hay varias)
    - Verás una tabla con datos de cookies
    - Haz clic derecho en la tabla → "Seleccionar todo" → clic derecho de nuevo → "Copiar"
    - Abre Notepad (Windows) o TextEdit (Mac)
    - Pega todo (Ctrl+V o Cmd+V)
    - Guarda el archivo como `cookies.txt` en tu Escritorio
    - Cambia el tipo de archivo a "Todos los archivos" al guardar (no .txt.txt)

2. **Usuarios de Firefox:**
    - Abre Firefox
    - Ve a https://addons.mozilla.org y busca "cookies.txt"
    - Instala la extensión "cookies.txt" (normalmente de "Rotemdan")
    - Ve a YouTube.com y asegúrate de estar conectado
    - Busca el nuevo icono de cookie en la barra de extensiones
    - Haz clic en el icono → "Current Site" → "Export"
    - Guarda el archivo como `cookies.txt` en tu Escritorio

3. **Usuarios de Safari:**
    - Sinceramente, usa Chrome o Firefox para este paso - Safari lo complica mucho
    - Si debes usar Safari: activa el menú de desarrollo (Safari → Preferencias → Avanzado → "Mostrar menú Desarrollo"), luego es similar a Chrome pero con más pasos

**Paso 2: Usa las cookies**

Ahora, cuando quieras descargar un video privado:

1. Copia la URL del video privado/no listado desde tu navegador
2. Abre la Terminal/Símbolo del sistema
3. Navega a donde quieras guardar el video (o irá a la carpeta "downloads")
4. Escribe este comando, reemplazando las partes en MAYÚSCULAS:
```bash
yt-cli -c ~/Desktop/cookies.txt "PON_LA_URL_DEL_VIDEO_AQUÍ"
```

**Ejemplo real:**
```bash
yt-cli -c ~/Desktop/cookies.txt "https://youtube.com/watch?v=PRIVATE_VIDEO_ID"
```

**Usuarios de Windows:** Usa este formato:
```bash
yt-cli -c C:\Users\TuUsuario\Desktop\cookies.txt "https://youtube.com/watch?v=PRIVATE_VIDEO_ID"
```
(Reemplaza "TuUsuario" por tu nombre real de usuario de Windows)

**Notas importantes:**
- El archivo de cookies debe ser reciente - si no funciona, exporta nuevas cookies
- Debes estar conectado a la misma cuenta que tiene acceso al video privado
- Esto funciona para videos no listados, privados, contenido solo para miembros, etc.

## Todas las opciones

```
yt-cli [OPCIONES] <URL>

OPCIONES:
-o, --output <DIR>    Dónde guardar los archivos (por defecto: ./downloads)
-a, --audio-only      Descargar solo el audio (formato mp3)
-p, --playlist        Descargar lista de reproducción completa
-q, --quality <Q>     Calidad de video: 144p, 240p, 360p, 480p, 720p, 1080p, 1440p, 2160p
-c, --cookies <ARCH>  Archivo de cookies para videos privados (ver instrucciones arriba)
-u, --username <EMAIL> Tu email de YouTube (alternativa a cookies)
--password <PASS>     Tu contraseña de YouTube (usar con username)
--netrc               Usar archivo .netrc para login (usuarios avanzados)
-h, --help            Mostrar ayuda
-V, --version         Mostrar versión
```

## Solución de problemas

**"yt-dlp not found":**  
- Saltaste el paso 1. Vuelve e instala yt-dlp primero
- Asegúrate de poder escribir `yt-dlp --version` y ver un número de versión

**"command not found: yt-cli":**  
- Abre una nueva ventana de terminal (cierra la anterior primero)
- Si sigue sin funcionar, prueba la ruta completa: `~/.cargo/bin/yt-cli --help`
- Puede que necesites agregar el directorio bin de Cargo a tu PATH

**"Invalid URL":**  
- Asegúrate de copiar la URL completa de YouTube desde tu navegador
- Debe empezar con https://youtube.com o https://youtu.be
- Elimina cualquier carácter extra que hayas copiado

**Falla video privado con cookies:**  
- Asegúrate de que tu archivo cookies.txt sea reciente (exporta uno nuevo)
- Verifica que estés conectado a la cuenta correcta de YouTube
- Verifica la ruta del archivo cookies.txt
- Asegúrate de que el archivo de cookies no esté vacío

**"cargo not found":**
- Saltaste la instalación de Rust. Ve a https://rustup.rs e instálalo primero
- Después de instalar, cierra la terminal y abre una nueva

**Descargas lentas:**  
- Es YouTube limitando la velocidad, no esta herramienta
- Intenta descargar en horarios de poco tráfico
- No se puede hacer nada - es intencional de YouTube

**Errores de permiso denegado:**  
- Asegúrate de tener permisos de escritura en la carpeta de salida
- Prueba usar otra carpeta como tu Escritorio: `yt-cli -o ~/Desktop`

**"Failed to start download":**  
- Asegúrate de que yt-dlp esté bien instalado y funcionando
- Prueba actualizar yt-dlp: `pip install --upgrade yt-dlp` o `brew upgrade yt-dlp`

## Licencia

MIT - haz lo que quieras con esto