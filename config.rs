// Status d'un job
enum State {
	RUNNING,
	STOPPED,
	EXITED,
	KILLED
}

// Enum des instructions à envoyer sur le task 
enum Instruction {
	START,
	RESTART,
	STOP,
	STATUS,
	SHUTDOWN
}

// enum des signaux
enum Signal {
	SIGHUP,
	SIGINT,
	SIGQUIT,
	SIGILL,
	SIGTRAP,
	SIGABRT,
	SIGBUS,
	SIGFPE,
	SIGKILL,
	SIGUSR1,
	SIGSEGV,
	SIGUSR2,
	SIGPIPE,
	SIGALRM,
	SIGTERM,
	SIGSTKFLT,
	SIGCHLD,
	SIGCONT,
	SIGSTOP,
	SIGTSTP,
	SIGTTIN,
	SIGTTOU,
	SIGURG
}

// Un enum pour les erreurs pas mal pour la gestion et centraliser les messages
enum Error {
	InvalidCmd
}

// Structure du gestionnaire de job control avec le fichier de conf
struct taskmaster {
	configFile: str,

}

// Structure de job avec la commande, le status, l'option d autorestart et le starttime (à completer)
struct Job {
	cmd: Cmd,
	state: State,
	autorestart: bool,
	starttime: temps(),
}
