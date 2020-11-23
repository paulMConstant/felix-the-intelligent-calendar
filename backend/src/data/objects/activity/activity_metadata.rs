use std::collections::HashSet;

/// Simple structure holding non-computation related data : id, name, participants.
///
/// We directly store incompatible activities in the ActivityComputationData which is why
/// the participants are not directly computation-related.
#[derive(Debug, Clone)]
pub struct ActivityMetadata {
    id: u16,
    name: String,
    participants: HashSet<String>,
}

impl ActivityMetadata {
    /// Creates new activity metadata.
    #[must_use]
    pub fn new<S>(id: u16, name: S) -> ActivityMetadata
    where
        S: Into<String>,
    {
        ActivityMetadata {
            id,
            name: name.into(),
            participants: HashSet::new(),
        }
    }

    // *** Getters ***

    /// Simple getter for the id.
    #[must_use]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Simple getter for the participants, sorted by name.
    #[must_use]
    pub fn participants_sorted(&self) -> Vec<String> {
        let mut participants_vec: Vec<String> = self.participants.iter().cloned().collect();
        participants_vec.sort();
        participants_vec
    }

    /// Getter for the participants, not sorted.
    pub fn participants_as_set(&self) -> &HashSet<String> {
        &self.participants
    }

    // *** Setters ***

    // No setter for the id. The id should be unique and never change.

    /// Simple setter for the name.
    ///
    /// The name is not checked or formatted. The activities collection does it.
    /// It is easier for the collection to do it because name formatting is done for addition of
    /// new activities, renaming, addition of entities to activities, etc.
    /// Having the activities collection do it keeps it in one place.
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.name = name.into();
    }

    /// Adds a participant to the activity.
    /// The participants are always sorted.
    ///
    /// # Errors
    ///
    /// Returns Err if the participant is already taking part in the activity.
    #[must_use]
    pub fn add_participant<S>(&mut self, participant: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        let participant = participant.into();
        if self.participants.insert(participant.clone()) {
            Ok(())
        } else {
            Err(format!(
                "{} is already taking part in the activity '{}' !",
                participant,
                self.name()
            ))
        }
    }

    /// Removes a participant from the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the participant is not taking part in the activity.
    #[must_use]
    pub fn remove_participant<S>(&mut self, participant: S) -> Result<(), String>
    where
        S: Into<String>,
    {
        let participant = participant.into();
        if self.participants.remove(&participant) {
            Ok(())
        } else {
            Err(format!(
                "{} is not taking part in the activity '{}' !",
                participant,
                self.name()
            ))
        }
    }

    /// Renames a participant in the activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the participant is not taking part in the activity or if
    /// a participant with the new name is already taking part in the activity.
    #[must_use]
    pub fn rename_participant<S1, S2>(&mut self, old_name: S1, new_name: S2) -> Result<(), String>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let old_name = old_name.into();
        let new_name = new_name.into();
        if self.participants.remove(&old_name) {
            if self.participants.insert(new_name.clone()) {
                Ok(())
            } else {
                self.participants.insert(old_name);
                Err(format!("{} already exists !", new_name))
            }
        } else {
            Err(format!(
                "{} is not taking part in the activity '{}' !",
                old_name,
                self.name()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_remove_participant() {
        let name = "Meta";
        let id = 0;
        let mut metadata = ActivityMetadata::new(id, name);

        // Add new participant
        let participant = "Participant";
        assert!(metadata.add_participant(participant.clone()).is_ok());
        assert_eq!(metadata.participants_sorted()[0], participant.clone());

        // Add the same participant again
        assert!(metadata.add_participant(participant.clone()).is_err());
        assert_eq!(metadata.participants_sorted().len(), 1);

        // Remove invalid participant
        let new_participant = "New participant";
        assert!(metadata
            .remove_participant(new_participant.clone())
            .is_err());

        // Add new participant && check sorting
        assert!(metadata.add_participant(new_participant.clone()).is_ok());
        let participants = metadata.participants_sorted();
        assert_eq!(participants[0], new_participant.clone());
        assert_eq!(participants[1], participant.clone());

        // Remove valid participant
        assert!(metadata.remove_participant(participant).is_ok());
        assert_eq!(metadata.participants_sorted().len(), 1)
    }

    #[test]
    fn rename_participant() {
        let name = "Meta";
        let id = 0;
        let mut metadata = ActivityMetadata::new(id, name);

        let participant = "Participant";
        let other_participant = "Other Participant";
        metadata.add_participant(participant.clone()).unwrap();
        metadata.add_participant(other_participant.clone()).unwrap();

        let new_name = "New Name";
        assert!(metadata
            .rename_participant("I Dont exist", new_name.clone())
            .is_err());
        assert!(metadata
            .rename_participant(participant.clone(), new_name.clone())
            .is_ok());

        assert_eq!(metadata.participants_sorted()[0], new_name);
        assert_eq!(metadata.participants_sorted()[1], other_participant);
    }
}
