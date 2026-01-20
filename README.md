# Kuse Cowork - Agente de IA para Escritorio

**Alternativa de código abierto a la aplicación de escritorio de Claude Code**

**Funciona con cualquier modelo, BYOK (Trae tu propia clave), escrito en Rust** 🚀

[*Video de demostración: Kuse Cowork en acción*](https://github.com/user-attachments/assets/e128e657-c1be-4134-828d-01a9a94ef055)

## ✨ ¿Por qué Kuse Cowork?

### 🔐 **BYOK (Trae tu propia clave)**
Utiliza tus propias claves API o incluso **trae tus propios modelos locales** para un control total de la privacidad.

### ⚡ **Agente Rust Puro**
Agente completamente escrito en Rust con **cero dependencias externas** (en tiempo de ejecución) - increíblemente rápido y seguro en memoria.

### 🌍 **Nativo Multiplataforma**
Rendimiento nativo real en macOS, Windows y Linux gracias a Tauri.

### 🛡️ **Aislamiento de Contenedores y Seguridad**
Utiliza contenedores Docker para la ejecución segura de comandos y un aislamiento completo del sistema host.

### 🧩 **Sistema de Habilidades Extensible**
Soporte para habilidades personalizadas para extender las capacidades del agente.
Las habilidades predeterminadas incluyen soporte para: docx, pdf, pptx, xlsx.

### 🔗 **Soporte de Protocolo MCP**
Soporte completo para el Protocolo de Contexto de Modelo (MCP) para una integración de herramientas sin problemas.

---

## 🚀 Características

- **🔒 Local y Privado**: Se ejecuta completamente en tu máquina, las llamadas a la API van directamente a tu proveedor elegido.
- **🔑 Soporte BYOK**: Usa tus propias APIs de Anthropic, OpenAI o modelos locales.
- **🎯 Agnóstico del Modelo**: Funciona con Claude, GPT, modelos locales (Ollama/LM Studio), y más.
- **🖥️ Multiplataforma**: macOS (ARM e Intel), Windows y Linux.
- **🪶 Ligero**: ~10MB de tamaño de aplicación usando Tauri.
- **🐳 Contenerizado**: Aislamiento Docker para seguridad mejorada.
- **🧩 Habilidades**: Sistema de habilidades extensible para capacidades personalizadas.
- **🔗 MCP**: Soporte del Protocolo de Contexto de Modelo para integración de herramientas.

## Nota de Seguridad
Este es un proyecto temprano. Por favor, ten mucho cuidado al conectar tus carpetas locales, especialmente con acceso de escritura. Se recomienda usar la funcionalidad de aislamiento de Docker.

## 📥 Instalación

### Opción 1: Descargar Binarios (Recomendado)
Ve a la sección de [Releases](https://github.com/kuse-ai/kuse-cowork/releases) y descarga el instalador apropiado para tu sistema:
- **macOS**: `.dmg` (Apple Silicon o Intel)
- **Windows**: `.msi`
- **Linux**: `.deb` o `.AppImage`

### Opción 2: Compilar desde el Código Fuente

#### Prerrequisitos
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (para Tauri)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (recomendado para aislamiento)

#### Pasos de Compilación

```bash
# Clonar el repositorio
git clone https://github.com/kuse-ai/kuse-cowork.git
cd kuse-cowork

# Instalar dependencias del frontend
npm install

# Ejecutar en modo desarrollo
npm run tauri dev

# Compilar para producción (generará el instalador en src-tauri/target/release/bundle)
npm run tauri build
```

---

## 🚀 Inicio Rápido

### 1. ⚙️ Configura tu Modelo de IA
1. Abre **Ajustes** (icono de engranaje en la barra lateral).
2. **Elige tu proveedor de IA:**
   - **Anthropic Claude** - Introduce tu clave API de Claude.
   - **OpenAI GPT** - Introduce tu clave API de OpenAI.
   - **Modelos Locales** - Configura el endpoint de Ollama/LM Studio.
3. **Selecciona tu modelo preferido** (Claude 3.5 Sonnet, GPT-4o, Llama 3, etc.).

### 2. 📁 Establece la Carpeta de Trabajo
- Haz clic en **"Select Project Path"** al crear una nueva tarea.
- Elige tu carpeta de proyecto o directorio de trabajo.
- El agente trabajará dentro de este contexto.

### 3. 🎯 ¡Comienza tu Primera Tarea!
1. Haz clic en **"New Task"**.
2. Describe lo que quieres lograr.
3. Observa cómo el agente de IA trabaja en tu proyecto.

**Ejemplos de tareas:**
- *"Organiza mis carpetas de descargas"*
- *"Lee todos los recibos en esta carpeta y haz un reporte de gastos"*
- *"Analiza el código y encuentra posibles bugs"*

---

## 🛠️ Desarrollo y Contribución

### Estructura del Proyecto

```
kuse-cowork/
├── src/                    # Frontend (SolidJS + TypeScript)
├── src-tauri/             # Backend (Rust + Tauri)
│   ├── src/               # Código fuente Rust
│   │   ├── agent/         # Lógica del agente
│   │   ├── tools/         # Herramientas (Bash, Docker, Archivos)
│   │   ├── skills/        # Sistema de habilidades
│   │   ├── mcp/           # Soporte MCP
│   └── Cargo.toml         # Dependencias Rust
└── docs/                  # Documentación
```

## 🛡️ Seguridad y Privacidad

### Aislamiento de Contenedores
Kuse Cowork puede usar contenedores Docker para aislar la ejecución de comandos externos:
- **Aislamiento completo** de tu sistema host.
- **Entornos limpios** para cada ejecución.

### Privacidad Primero
- **Sin telemetría**: Nada se envía a nuestros servidores.
- **Almacenamiento local**: Todos los datos permanecen en tu máquina.
- **Código abierto**: Transparencia total.

## 📄 Licencia

MIT License - ver [LICENSE](LICENSE) para más detalles.

## 🙏 Créditos

Desarrollado por **alarti**.

Inspirado por:
- **[Claude Cowork](https://claude.com/blog/cowork-research-preview)**
