# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_myth_global_optspecs
	string join \n c/config= l/log-level= no-tui no-sandbox h/help V/version
end

function __fish_myth_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_myth_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_myth_using_subcommand
	set -l cmd (__fish_myth_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c myth -n "__fish_myth_needs_command" -s c -l config -d 'Path to user config file (default: ~/.config/myth/user.yaml)' -r
complete -c myth -n "__fish_myth_needs_command" -s l -l log-level -d 'Override log level (trace, debug, info, warn, error)' -r -a "trace debug info warn error"
complete -c myth -n "__fish_myth_needs_command" -l no-tui -d 'Disable TUI (use simple interactive CLI)'
complete -c myth -n "__fish_myth_needs_command" -l no-sandbox -d 'Disable sandbox (NOT RECOMMENDED)'
complete -c myth -n "__fish_myth_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c myth -n "__fish_myth_needs_command" -s V -l version -d 'Print version'
complete -c myth -n "__fish_myth_needs_command" -f -a "scan" -d 'Initialize full-spectrum target acquisition and reconnaissance'
complete -c myth -n "__fish_myth_needs_command" -f -a "stealth" -d 'Launch immediate low-signature, passive-only reconnaissance'
complete -c myth -n "__fish_myth_needs_command" -f -a "osint" -d 'Launch specialized Open Source Intelligence (OSINT) ops'
complete -c myth -n "__fish_myth_needs_command" -f -a "vuln" -d 'Perform a deep, multi-vector vulnerability assessment'
complete -c myth -n "__fish_myth_needs_command" -f -a "tools" -d 'Catalog and display all synchronized mission assets/tools'
complete -c myth -n "__fish_myth_needs_command" -f -a "target" -d 'Force rotate the mission focus to a new target/CIDR'
complete -c myth -n "__fish_myth_needs_command" -f -a "chat" -d 'Launch an interactive tactical chat session with the agent'
complete -c myth -n "__fish_myth_needs_command" -f -a "config" -d 'Retrieve the current mission configuration metadata'
complete -c myth -n "__fish_myth_needs_command" -f -a "profile" -d 'View or modulate tactical reconnaissance profiles/phases'
complete -c myth -n "__fish_myth_needs_command" -f -a "check" -d 'Verify system health, sandbox status, and tool availability'
complete -c myth -n "__fish_myth_needs_command" -f -a "mcp" -d 'Manage Custom User MCP Servers (Local & Remote)'
complete -c myth -n "__fish_myth_needs_command" -f -a "vitals" -d 'Analyze neural pulses and session lifecycle metadata'
complete -c myth -n "__fish_myth_needs_command" -f -a "findings" -d 'Aggregate and display all discovered tactical intelligence'
complete -c myth -n "__fish_myth_needs_command" -f -a "graph" -d 'Render the infrastructure relationship graph of target assets'
complete -c myth -n "__fish_myth_needs_command" -f -a "history" -d 'Aggregate tactical event logs and mission history'
complete -c myth -n "__fish_myth_needs_command" -f -a "report" -d 'Generate a comprehensive executive intelligence summary'
complete -c myth -n "__fish_myth_needs_command" -f -a "sync" -d 'Force a re-synchronization with local tool registries'
complete -c myth -n "__fish_myth_needs_command" -f -a "burn" -d 'EMERGENCY: Immediate shred of all data and system shutdown'
complete -c myth -n "__fish_myth_needs_command" -f -a "wipe" -d 'Wipe the current session memory and tactical context'
complete -c myth -n "__fish_myth_needs_command" -f -a "clear" -d 'Purge visual buffers (Memory remains)'
complete -c myth -n "__fish_myth_needs_command" -f -a "depth" -d 'Modulate the maximum neural iteration depth for the agent'
complete -c myth -n "__fish_myth_needs_command" -f -a "inspect" -d 'Retrieve deep technical documentation for a specific asset'
complete -c myth -n "__fish_myth_needs_command" -f -a "completions" -d 'Generate high-performance shell autocompletion tactical scripts'
complete -c myth -n "__fish_myth_needs_command" -f -a "usage" -d 'Display the tactical usage documentation'
complete -c myth -n "__fish_myth_needs_command" -f -a "version" -d 'Display the current neural core version'
complete -c myth -n "__fish_myth_needs_command" -f -a "subdomains" -d 'High-speed multi-source subdomain discovery'
complete -c myth -n "__fish_myth_needs_command" -f -a "master" -d 'ULTRA-ROBUST DISCOVERY: Tor + Proxies + Mega Wordlist + Deep Recursion'
complete -c myth -n "__fish_myth_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c myth -n "__fish_myth_using_subcommand scan" -s p -l profile -d 'Reconnaissance methodology profile' -r
complete -c myth -n "__fish_myth_using_subcommand scan" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand stealth" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand osint" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand vuln" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand tools" -s c -l category -d 'Filter mission assets by technical category' -r
complete -c myth -n "__fish_myth_using_subcommand tools" -s s -l search -d 'Search mission assets by name or keyword' -r
complete -c myth -n "__fish_myth_using_subcommand tools" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand target" -s p -l profile -d 'Reconnaissance methodology profile' -r
complete -c myth -n "__fish_myth_using_subcommand target" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand chat" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand config" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand profile" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand check" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "list" -d 'List all configured MCP servers'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "toggle" -d 'Toggle an MCP server (enable/disable)'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "add-local" -d 'Add a new Local MCP Server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "add-remote" -d 'Add a new Remote MCP Server via SSE'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "tools" -d 'List all available tools for an MCP server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "sync" -d 'Force re-sync factory defaults to mcp.json'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "remove" -d 'Remove an MCP server from the registry'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "allow-tool" -d 'Allow a specific tool from an MCP server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "block-tool" -d 'Block a specific tool from an MCP server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and not __fish_seen_subcommand_from list toggle add-local add-remote tools sync remove allow-tool block-tool help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from toggle" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-local" -s a -l args -d 'Arguments for the command (comma-separated or multiple flags)' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-local" -s e -l env -d 'Environment variables (KEY=VALUE, comma-separated)' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-local" -s d -l dir -d 'Working directory' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-local" -l description -d 'Operational description' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-local" -s t -l transport -d 'Transport protocol (stdio, sse, http). Default: stdio' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-local" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-remote" -l headers -d 'Custom headers (Header:Value, comma-separated)' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-remote" -l description -d 'Operational description' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-remote" -s t -l transport -d 'Transport protocol (sse, http, stdio). Default: sse' -r
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from add-remote" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from tools" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from sync" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from allow-tool" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from block-tool" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "list" -d 'List all configured MCP servers'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "toggle" -d 'Toggle an MCP server (enable/disable)'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "add-local" -d 'Add a new Local MCP Server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "add-remote" -d 'Add a new Remote MCP Server via SSE'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "tools" -d 'List all available tools for an MCP server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "sync" -d 'Force re-sync factory defaults to mcp.json'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove an MCP server from the registry'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "allow-tool" -d 'Allow a specific tool from an MCP server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "block-tool" -d 'Block a specific tool from an MCP server'
complete -c myth -n "__fish_myth_using_subcommand mcp; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c myth -n "__fish_myth_using_subcommand vitals" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand findings" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand graph" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand history" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand report" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand sync" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand burn" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand wipe" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand clear" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand depth" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand inspect" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand completions" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand usage" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand version" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand subdomains" -l active -d 'Enable active brute-force and permutations'
complete -c myth -n "__fish_myth_using_subcommand subdomains" -l recursive -d 'Enable recursive discovery'
complete -c myth -n "__fish_myth_using_subcommand subdomains" -l only-alive -d 'Filter results to only show live subdomains (Default: true)'
complete -c myth -n "__fish_myth_using_subcommand subdomains" -l master -d 'ULTRA-ROBUST MODE: Tor + Proxies + Mega Wordlist + Deep Recursion'
complete -c myth -n "__fish_myth_using_subcommand subdomains" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand master" -s h -l help -d 'Print help'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "scan" -d 'Initialize full-spectrum target acquisition and reconnaissance'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "stealth" -d 'Launch immediate low-signature, passive-only reconnaissance'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "osint" -d 'Launch specialized Open Source Intelligence (OSINT) ops'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "vuln" -d 'Perform a deep, multi-vector vulnerability assessment'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "tools" -d 'Catalog and display all synchronized mission assets/tools'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "target" -d 'Force rotate the mission focus to a new target/CIDR'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "chat" -d 'Launch an interactive tactical chat session with the agent'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "config" -d 'Retrieve the current mission configuration metadata'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "profile" -d 'View or modulate tactical reconnaissance profiles/phases'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "check" -d 'Verify system health, sandbox status, and tool availability'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "mcp" -d 'Manage Custom User MCP Servers (Local & Remote)'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "vitals" -d 'Analyze neural pulses and session lifecycle metadata'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "findings" -d 'Aggregate and display all discovered tactical intelligence'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "graph" -d 'Render the infrastructure relationship graph of target assets'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "history" -d 'Aggregate tactical event logs and mission history'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "report" -d 'Generate a comprehensive executive intelligence summary'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "sync" -d 'Force a re-synchronization with local tool registries'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "burn" -d 'EMERGENCY: Immediate shred of all data and system shutdown'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "wipe" -d 'Wipe the current session memory and tactical context'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "clear" -d 'Purge visual buffers (Memory remains)'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "depth" -d 'Modulate the maximum neural iteration depth for the agent'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "inspect" -d 'Retrieve deep technical documentation for a specific asset'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "completions" -d 'Generate high-performance shell autocompletion tactical scripts'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "usage" -d 'Display the tactical usage documentation'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "version" -d 'Display the current neural core version'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "subdomains" -d 'High-speed multi-source subdomain discovery'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "master" -d 'ULTRA-ROBUST DISCOVERY: Tor + Proxies + Mega Wordlist + Deep Recursion'
complete -c myth -n "__fish_myth_using_subcommand help; and not __fish_seen_subcommand_from scan stealth osint vuln tools target chat config profile check mcp vitals findings graph history report sync burn wipe clear depth inspect completions usage version subdomains master help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "list" -d 'List all configured MCP servers'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "toggle" -d 'Toggle an MCP server (enable/disable)'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "add-local" -d 'Add a new Local MCP Server'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "add-remote" -d 'Add a new Remote MCP Server via SSE'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "tools" -d 'List all available tools for an MCP server'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "sync" -d 'Force re-sync factory defaults to mcp.json'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "remove" -d 'Remove an MCP server from the registry'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "allow-tool" -d 'Allow a specific tool from an MCP server'
complete -c myth -n "__fish_myth_using_subcommand help; and __fish_seen_subcommand_from mcp" -f -a "block-tool" -d 'Block a specific tool from an MCP server'
