# Seguridad en Kuse Cowork

Este documento detalla el modelo de seguridad de Kuse Cowork, enfocándose en la ejecución de comandos y el acceso al sistema de archivos.

## Modelo de Ejecución

Kuse Cowork permite al agente ejecutar comandos de shell para cumplir con las tareas solicitadas. Existen dos modos de ejecución:

### 1. Ejecución en Host (Precaución)

Cuando el agente utiliza la herramienta `bash`, los comandos se ejecutan **directamente en su sistema operativo host** con los mismos privilegios que el usuario que inició la aplicación.

*   **Riesgos**:
    *   Acceso irrestricto a archivos personales.
    *   Posibilidad de daño accidental o malicioso al sistema (e.g., `rm -rf`).
    *   Ejecución de scripts no verificados.

*   **Mitigaciones**:
    *   La herramienta `bash` tiene una lista negra de patrones peligrosos (`rm -rf /`, bombas fork, etc.).
    *   Se recomienda encarecidamente supervisar las acciones del agente.

### 2. Ejecución Aislada (Docker)

Cuando el agente utiliza las herramientas `docker_*`, los comandos se ejecutan dentro de contenedores efímeros.

*   **Beneficios**:
    *   **Aislamiento**: El entorno está separado del host.
    *   **Control de Acceso**: Solo los directorios explícitamente montados son accesibles.
    *   **Entorno Limpio**: Cada ejecución puede partir de un estado conocido.

*   **Configuración**:
    *   Se requiere tener Docker Desktop instalado y ejecutándose.
    *   El agente monta el directorio del proyecto actual (`/workspace`) y el directorio de habilidades (`/skills`).

## Acceso a Archivos

El agente tiene capacidad de lectura y escritura (`read_file`, `write_file`, `edit_file`).

*   El acceso se limita principalmente al directorio de trabajo seleccionado por el usuario.
*   Sin embargo, debido a la naturaleza de las rutas absolutas, técnicamente es posible acceder a otras ubicaciones si el agente lo intenta explícitamente.

## Recomendaciones de Seguridad

1.  **Utilice Docker siempre que sea posible**: Pida al agente que ejecute tareas complejas o scripts desconocidos dentro de un contenedor.
2.  **Revise el Plan**: Antes de que el agente ejecute acciones, revise el plan propuesto.
3.  **Respalde sus Datos**: Asegúrese de tener copias de seguridad de los proyectos en los que trabaja el agente.
4.  **No ejecute como Root/Admin**: Ejecute la aplicación con un usuario estándar.

## Reporte de Vulnerabilidades

Si encuentra una vulnerabilidad de seguridad, por favor repórtela abriendo un Issue en el repositorio o contactando a los desarrolladores directamente.
