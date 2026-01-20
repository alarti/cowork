# Arquitectura de Kuse Cowork

Kuse Cowork es una aplicación de escritorio construida sobre una arquitectura híbrida moderna que combina la flexibilidad de la web con el rendimiento y seguridad de Rust.

## Stack Tecnológico

*   **Frontend**: SolidJS + TypeScript + Vite
*   **Backend**: Rust + Tauri v2
*   **Base de Datos**: SQLite (local)
*   **Aislamiento**: Docker

## Diagrama de Componentes

```mermaid
graph TD
    UI[Frontend (SolidJS)] <-->|Tauri IPC| Core[Backend (Rust)]
    Core --> Agent[Agente IA]
    Core --> DB[(SQLite)]
    Core --> MCP[Cliente MCP]

    Agent --> LLM[Proveedor LLM (Claude/OpenAI)]
    Agent --> Tools[Herramientas]

    Tools --> Bash[Host Shell]
    Tools --> FS[Sistema de Archivos]
    Tools --> Docker[Docker Engine]

    MCP <--> MCPServers[Servidores MCP Externos]
```

## Flujo del Agente (Agent Loop)

El núcleo de la aplicación es el bucle del agente (`src-tauri/src/agent/agent_loop.rs`), que sigue este proceso:

1.  **Recepción de Tarea**: El usuario envía un mensaje/tarea desde el frontend.
2.  **Construcción de Contexto**: Se recopila el historial de chat, el prompt del sistema y las definiciones de herramientas disponibles.
3.  **Llamada al LLM**: Se envía la solicitud al proveedor de IA configurado (Claude, OpenAI, etc.).
4.  **Procesamiento de Respuesta**:
    *   Si es texto: Se transmite al usuario.
    *   Si es uso de herramienta: Se identifica la herramienta y los parámetros.
5.  **Ejecución de Herramienta**:
    *   `ToolExecutor` determina si es una herramienta interna (bash, fs) o externa (MCP).
    *   Se ejecuta la acción y se captura el resultado (stdout/stderr).
6.  **Iteración**: El resultado de la herramienta se añade al historial y se vuelve al paso 3, permitiendo al agente reaccionar y continuar trabajando.

## Protocolo de Contexto de Modelo (MCP)

Kuse Cowork implementa el cliente MCP, permitiendo conectar "servidores" externos que exponen herramientas adicionales.

*   **Gestor MCP**: Administra las conexiones y el ciclo de vida de los servidores MCP.
*   **Integración**: Las herramientas MCP se inyectan dinámicamente en el contexto del agente, haciéndolas indistinguibles de las herramientas nativas para el LLM.

## Persistencia

*   **SQLite**: Almacena conversaciones, mensajes, tareas y configuración de servidores MCP.
*   **Tauri Store**: Almacena preferencias de usuario simples (tema, claves API, modelo seleccionado).
