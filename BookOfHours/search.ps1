# Declare $Mode in the global scope to ensure it persists across function calls
$global:Mode = ""
$global:cultureInfo = [System.Globalization.CultureInfo]::CurrentCulture
$global:textInfo = $cultureInfo.TextInfo

function Get-Queries {
    param(
        [string]$Mode,
        [string]$Query
    )

    # Base pattern to find the primary identifier field (Label or id):
    # We'll try a unified approach: we consider that any of the keys (.Label, .label, .id) could be used.
    # `select(. != null)` ensures we skip null fields, and test("$Query"; "i") ensures case-insensitive matching.
    $baseSelector = "((.Label // .label) // .id) | select(. != null) | test(""$Query""; ""i"")"
    Write-Host "DEBUG: Base Selector - $baseSelector"

    $nameSelector = "(.Label // .label // .id)"

    $NameQuery = ".[][] | select($baseSelector) | $nameSelector | select(.)"

    $ValueQuery = ".[][] | select($baseSelector) | (.desc // .Desc) | select(.)"

    $ObjectQuery = ".[][] | select($baseSelector)"

    switch ($Mode.ToLower()) {
        "skills" {
            $FilePath = ".\elements\skills.json"
        }
        "aspects" {
            $FilePath = ".\elements\_aspects.json"
        }
        "contamination aspects" {
            $FilePath = ".\elements\contamination_aspects.json"
        }
        "tomes" {
            $FilePath = ".\elements\tomes.json"
        }
        "aspected items" {
            $FilePath = ".\elements\aspecteditems.json"
        }
        default {
            Write-Error "Invalid Mode: $Mode. Supported modes are 'skills', 'aspects', 'contamination aspects'."
            return $null
        }
    }

    Write-Host "DEBUG (Get Queries): Object Query - $ObjectQuery"


    return [PSCustomObject]@{
        FilePath    = $FilePath
        NameQuery   = $NameQuery
        ValueQuery  = $ValueQuery
        ObjectQuery = $ObjectQuery
    }
}

# Utility Function to Fetch and Display Data
function Fetch-And-Display {
    param (
        [string]$FilePath,        # Path to the JSON file
        [string]$NameQuery,       # Query for the Name field
        [string]$ValueQuery,      # Query for the Value field
        [string]$ObjectQuery,     # Query for the full object
        [bool]$includeObjectQuery = $false  # Flag to include object query in result
    )
    
    # Validate Query
    if ([string]::IsNullOrWhiteSpace($NameQuery) -or [string]::IsNullOrWhiteSpace($ValueQuery)) {
        Write-Host "Error: Name or Value Query cannot be empty or whitespace-only." -ForegroundColor Red
        return
    }

    # Fetch Name and Value
    $NAME = $(Get-Content $FilePath | xq -r $NameQuery)
    $VALUE = $(Get-Content $FilePath | xq -r $ValueQuery)

    # Handle Object Query (only if -o flag is set)
    $OBJECT = ""
    if ($includeObjectQuery) {
        Write-Host "DEBUG (Process Object Query): Object Query - $ObjectQuery"

        $OBJECT = $(Get-Content $FilePath | xq -C $ObjectQuery | Out-String)
    }

    # Handle no results
    if ([string]::IsNullOrWhiteSpace($NAME) -and [string]::IsNullOrWhiteSpace($VALUE)) {
        Write-Host "No results found for the query."
    } else {
        $TAB = $([char]9)
        $COMBINED = "$NAME$TAB$VALUE"
        $COMBINED | Set-Clipboard
        Write-Host "Name: $NAME`nDescription: $VALUE"
        Write-Host "Excel-friendly output sent to clipboard."

        # Object Query Output (if -o flag is set)
        if ($includeObjectQuery) {
            Write-Host "`nFull Object:`n$OBJECT"
        }
    }
}



function skills {
    param (
        [string]$query,
        [bool]$includeObjectQuery = $false
    )

    $queries = Get-Queries -Mode "skills" -Query $query
    if ($null -eq $queries) { return }

    Fetch-And-Display -FilePath $queries.FilePath `
                      -NameQuery $queries.NameQuery `
                      -ValueQuery $queries.ValueQuery `
                      -ObjectQuery $queries.ObjectQuery `
                      -includeObjectQuery $includeObjectQuery
}

function aspects {
    param (
        [string]$query,
        [bool]$includeObjectQuery = $false
    )

    $queries = Get-Queries -Mode "aspects" -Query $query
    if ($null -eq $queries) { return }

    Fetch-And-Display -FilePath $queries.FilePath `
                      -NameQuery $queries.NameQuery `
                      -ValueQuery $queries.ValueQuery `
                      -ObjectQuery $queries.ObjectQuery `
                      -includeObjectQuery $includeObjectQuery
}

function contamination_aspects {
    param (
        [string]$query,
        [bool]$includeObjectQuery = $false
    )

    $queries = Get-Queries -Mode "contamination aspects" -Query $query
    if ($null -eq $queries) { return }

    Fetch-And-Display -FilePath $queries.FilePath `
                      -NameQuery $queries.NameQuery `
                      -ValueQuery $queries.ValueQuery `
                      -ObjectQuery $queries.ObjectQuery `
                      -includeObjectQuery $includeObjectQuery
}

function tomes {
    param (
        [string]$query,
        [bool]$includeObjectQuery = $false
    )

    $queries = Get-Queries -Mode "tomes" -Query $query
    if ($null -eq $queries) { return }

    Fetch-And-Display -FilePath $queries.FilePath `
                      -NameQuery $queries.NameQuery `
                      -ValueQuery $queries.ValueQuery `
                      -ObjectQuery $queries.ObjectQuery `
                      -includeObjectQuery $includeObjectQuery
}

function aspected_items {
    param (
        [string]$query,
        [bool]$includeObjectQuery = $false
    )

    $queries = Get-Queries -Mode "aspected items" -Query $query
    if ($null -eq $queries) { return }

    Fetch-And-Display -FilePath $queries.FilePath `
                      -NameQuery $queries.NameQuery `
                      -ValueQuery $queries.ValueQuery `
                      -ObjectQuery $queries.ObjectQuery `
                      -includeObjectQuery $includeObjectQuery
}


# Main REPL Loop with Mode Selection and CTRL+C Handling
function Select-Mode {
    while ($true) {
        $global:Mode = Read-Host "Enter mode (skills, aspects, contamination aspects, tomes, aspected items) or 'exit' to quit"

        if ($global:Mode.ToLower() -eq 'exit') {
            Write-Host "Exiting..."
            return $false
        }

        if ($global:Mode.ToLower() -in @("skills", "aspects", "contamination aspects", "tomes", "aspected items")) {
            Write-Host "Mode set to '$global:Mode'. Type your search query."
            return $true
        }

        Write-Host "Invalid mode. Please select a valid mode."
    }
}


# Initial Mode Selection
$continue = Select-Mode
if (-not $continue) { exit }

# Main loop
while ($true) {
    # Prompt user for query input
    $query = Read-Host -Prompt "Enter your search query or type 'reset' to change mode"

    # Handle exit
    if ($query.ToLower() -eq 'exit') {
        Write-Host "Exiting..." -ForegroundColor Green
        break
    }

    # Allow user to reset the mode selection
    if ($query.ToLower() -eq 'reset') {
        Write-Host "Returning to mode selection..."
        $continue = Select-Mode
        if (-not $continue) { break }
        continue
    }

    # Set flag for object query
    $includeObjectQuery = $false

    # Handle object flag (-o)
    if ($query -like "* -o") {
        $includeObjectQuery = $true
        $query = $query -replace " -o", ""  # Remove the flag from the query
    }

     # trim trailing whitespace so it don't mess up queries
    $query = $query.Trim()

    # Show the current mode and flags
    # Write-Host "DEBUG: Current mode: $Mode, Include Object Query: $includeObjectQuery"

    # Run the search based on the selected mode
    switch ($Mode.ToLower()) {
        "skills" {
            skills -query $query -includeObjectQuery $includeObjectQuery
        }
        "aspects" {
            aspects -query $query -includeObjectQuery $includeObjectQuery
        }
        "contamination aspects" {
            contamination_aspects -query $query -includeObjectQuery $includeObjectQuery
        }
        "tomes" {
            tomes -query $query -includeObjectQuery $includeObjectQuery
        }
        "aspected items" {
            aspected_items -query $query -includeObjectQuery $includeObjectQuery
        }
        default {
            Write-Host "Invalid mode. Please select a valid mode using 'reset'." -ForegroundColor Red
        }
    }
}
