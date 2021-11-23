package agora

// A Universe represents a set of all Characters, Locations and Events related between them
type Universe struct {
	Id          string `json:"id,omitempty"`
	Name        string `json:"name,omitempty"`
	User        string `json:"user,omitempty"`
	Description string `json:"description,omitempty"`
}

// A Moment is the minimum unity of time for a given universe
type Moment struct {
	Id       string   `json:"id,omitempty"`
	Date     string   `json:"date,omitempty"`
	Universe Universe `json:"universe,omitempty"`
	Before   *Moment  `json:"before,omitempty"`
	After    *Moment  `json:"after,omitempty"`
}
