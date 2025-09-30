use crate::vm::instruction::Opcode;
use crate::vm::types::Value;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    O1,
    O2,
    O3,
}

impl fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizationLevel::None => write!(f, "None"),
            OptimizationLevel::O1 => write!(f, "O1"),
            OptimizationLevel::O2 => write!(f, "O2"),
            OptimizationLevel::O3 => write!(f, "O3"),
        }
    }
}

/// Type profile for tracking runtime type information
#[derive(Debug, Clone)]
pub struct TypeProfile {
    type_counts: HashMap<String, u64>,
    total_observations: u64,
}

impl TypeProfile {
    pub fn new() -> Self {
        Self {
            type_counts: HashMap::new(),
            total_observations: 0,
        }
    }
    
    pub fn record_observation(&mut self, type_name: &str) {
        *self.type_counts.entry(type_name.to_string()).or_insert(0) += 1;
        self.total_observations += 1;
    }
    
    pub fn total_observations(&self) -> u64 {
        self.total_observations
    }
    
    pub fn get_type_frequency(&self, type_name: &str) -> u64 {
        self.type_counts.get(type_name).copied().unwrap_or(0)
    }
    
    pub fn is_monomorphic(&self, threshold: f64) -> bool {
        if self.total_observations == 0 {
            return false;
        }
        
        let max_count = self.type_counts.values().max().copied().unwrap_or(0);
        let ratio = max_count as f64 / self.total_observations as f64;
        ratio >= threshold
    }
}

impl Default for TypeProfile {
    fn default() -> Self {
        Self::new()
    }
}

/// Branch profile for tracking conditional branch behavior
#[derive(Debug, Clone)]
pub struct BranchProfile {
    taken_count: u64,
    not_taken_count: u64,
}

impl BranchProfile {
    pub fn new() -> Self {
        Self {
            taken_count: 0,
            not_taken_count: 0,
        }
    }
    
    pub fn record_branch(&mut self, taken: bool) {
        if taken {
            self.taken_count += 1;
        } else {
            self.not_taken_count += 1;
        }
    }
    
    pub fn total_branches(&self) -> u64 {
        self.taken_count + self.not_taken_count
    }
    
    pub fn taken_count(&self) -> u64 {
        self.taken_count
    }
    
    pub fn not_taken_count(&self) -> u64 {
        self.not_taken_count
    }
    
    pub fn taken_percentage(&self) -> f64 {
        if self.total_branches() == 0 {
            0.0
        } else {
            self.taken_count as f64 / self.total_branches() as f64
        }
    }
    
    pub fn predict_taken(&self) -> bool {
        self.taken_percentage() > 0.5
    }
}

impl Default for BranchProfile {
    fn default() -> Self {
        Self::new()
    }
}

/// Profile information for a specific instruction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfiledInstruction {
    pub pc: usize,
    pub opcode: Opcode,
    pub execution_count: u64,
}

impl ProfiledInstruction {
    pub fn new(pc: usize, opcode: Opcode) -> Self {
        Self {
            pc,
            opcode,
            execution_count: 0,
        }
    }
}

/// Hot spot profiler for tracking execution patterns and guiding JIT compilation
pub struct HotSpotProfiler {
    // Function execution tracking
    function_counts: HashMap<usize, u64>,
    function_threshold: u64,
    
    // Loop execution tracking
    loop_counts: HashMap<usize, u64>,
    loop_threshold: u64,
    
    // Type profiling
    type_profiles: HashMap<usize, TypeProfile>,
    
    // Branch profiling
    branch_profiles: HashMap<usize, BranchProfile>,
    
    // Instruction profiling
    instruction_profiles: HashMap<usize, ProfiledInstruction>,
    
    // Deoptimization tracking
    deoptimization_counts: HashMap<usize, u32>,
    deoptimization_reasons: HashMap<usize, Vec<String>>,
    
    // Total execution counter
    total_executions: u64,
}

impl HotSpotProfiler {
    pub fn new() -> Self {
        Self {
            function_counts: HashMap::new(),
            function_threshold: 1000,
            loop_counts: HashMap::new(),
            loop_threshold: 10000,
            type_profiles: HashMap::new(),
            branch_profiles: HashMap::new(),
            instruction_profiles: HashMap::new(),
            deoptimization_counts: HashMap::new(),
            deoptimization_reasons: HashMap::new(),
            total_executions: 0,
        }
    }
    
    pub fn with_thresholds(function_threshold: u64, loop_threshold: u64) -> Self {
        Self {
            function_counts: HashMap::new(),
            function_threshold,
            loop_counts: HashMap::new(),
            loop_threshold,
            type_profiles: HashMap::new(),
            branch_profiles: HashMap::new(),
            instruction_profiles: HashMap::new(),
            deoptimization_counts: HashMap::new(),
            deoptimization_reasons: HashMap::new(),
            total_executions: 0,
        }
    }
    
    // Function execution tracking
    pub fn record_function_entry(&mut self, function_id: usize) {
        *self.function_counts.entry(function_id).or_insert(0) += 1;
        self.total_executions += 1;
    }
    
    pub fn get_function_count(&self, function_id: usize) -> u64 {
        self.function_counts.get(&function_id).copied().unwrap_or(0)
    }
    
    pub fn hot_functions(&self) -> Vec<usize> {
        self.function_counts
            .iter()
            .filter(|&(_, &count)| count >= self.function_threshold)
            .map(|(&id, _)| id)
            .collect()
    }
    
    // Loop execution tracking
    pub fn record_loop_iteration(&mut self, loop_pc: usize) {
        *self.loop_counts.entry(loop_pc).or_insert(0) += 1;
        self.total_executions += 1;
    }
    
    pub fn get_loop_count(&self, loop_pc: usize) -> u64 {
        self.loop_counts.get(&loop_pc).copied().unwrap_or(0)
    }
    
    pub fn hot_loops(&self) -> Vec<usize> {
        self.loop_counts
            .iter()
            .filter(|&(_, &count)| count >= self.loop_threshold)
            .map(|(&pc, _)| pc)
            .collect()
    }
    
    // Type profiling
    pub fn record_type_observation(&mut self, pc: usize, value: &Value) {
        let type_name = value.type_name();
        self.type_profiles
            .entry(pc)
            .or_default()
            .record_observation(type_name);
    }
    
    pub fn get_type_profile(&self, pc: usize) -> Option<&TypeProfile> {
        self.type_profiles.get(&pc)
    }
    
    // Branch profiling
    pub fn record_branch_taken(&mut self, pc: usize, taken: bool) {
        self.branch_profiles
            .entry(pc)
            .or_default()
            .record_branch(taken);
    }
    
    pub fn get_branch_profile(&self, pc: usize) -> Option<&BranchProfile> {
        self.branch_profiles.get(&pc)
    }
    
    // Instruction execution tracking
    pub fn record_instruction_execution(&mut self, pc: usize, opcode: Opcode) {
        let profile = self.instruction_profiles
            .entry(pc)
            .or_insert_with(|| ProfiledInstruction::new(pc, opcode));
        profile.execution_count += 1;
    }
    
    pub fn get_instruction_profile(&self, pc: usize) -> Option<&ProfiledInstruction> {
        self.instruction_profiles.get(&pc)
    }
    
    pub fn get_hot_instructions(&self, threshold: u64) -> Vec<&ProfiledInstruction> {
        self.instruction_profiles
            .values()
            .filter(|profile| profile.execution_count >= threshold)
            .collect()
    }
    
    // Optimization level suggestions
    pub fn suggested_optimization_level(&self, function_id: usize) -> OptimizationLevel {
        let count = self.get_function_count(function_id);
        
        match count {
            0..=50 => OptimizationLevel::None,
            51..=500 => OptimizationLevel::O1,
            501..=5000 => OptimizationLevel::O2,
            _ => OptimizationLevel::O3,
        }
    }
    
    // Deoptimization tracking
    pub fn record_deoptimization(&mut self, pc: usize, reason: &str) {
        *self.deoptimization_counts.entry(pc).or_insert(0) += 1;
        self.deoptimization_reasons
            .entry(pc)
            .or_default()
            .push(reason.to_string());
    }
    
    pub fn get_deoptimization_count(&self, pc: usize) -> u32 {
        self.deoptimization_counts.get(&pc).copied().unwrap_or(0)
    }
    
    pub fn should_avoid_optimization(&self, pc: usize, threshold: u32) -> bool {
        self.get_deoptimization_count(pc) >= threshold
    }
    
    // General statistics
    pub fn total_executions(&self) -> u64 {
        self.total_executions
    }
    
    // Profile data export/import
    pub fn export_profile_data(&self) -> String {
        let data = ProfileData {
            function_counts: self.function_counts.clone(),
            loop_counts: self.loop_counts.clone(),
            type_profiles: self.serialize_type_profiles(),
            branch_profiles: self.serialize_branch_profiles(),
        };
        
        serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string())
    }
    
    pub fn import_profile_data(&mut self, data: &str) -> Result<(), String> {
        let profile_data: ProfileData = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse profile data: {}", e))?;
        
        self.function_counts = profile_data.function_counts;
        self.loop_counts = profile_data.loop_counts;
        self.deserialize_type_profiles(profile_data.type_profiles);
        self.deserialize_branch_profiles(profile_data.branch_profiles);
        
        Ok(())
    }
    
    // Reset all profiling data
    pub fn reset(&mut self) {
        self.function_counts.clear();
        self.loop_counts.clear();
        self.type_profiles.clear();
        self.branch_profiles.clear();
        self.instruction_profiles.clear();
        self.deoptimization_counts.clear();
        self.deoptimization_reasons.clear();
        self.total_executions = 0;
    }
    
    // Helper methods for serialization
    fn serialize_type_profiles(&self) -> HashMap<String, HashMap<String, u64>> {
        let mut result = HashMap::new();
        for (pc, profile) in &self.type_profiles {
            result.insert(pc.to_string(), profile.type_counts.clone());
        }
        result
    }
    
    fn deserialize_type_profiles(&mut self, data: HashMap<String, HashMap<String, u64>>) {
        self.type_profiles.clear();
        for (pc_str, type_counts) in data {
            if let Ok(pc) = pc_str.parse::<usize>() {
                let mut profile = TypeProfile::new();
                profile.type_counts = type_counts;
                profile.total_observations = profile.type_counts.values().sum();
                self.type_profiles.insert(pc, profile);
            }
        }
    }
    
    fn serialize_branch_profiles(&self) -> HashMap<String, (u64, u64)> {
        let mut result = HashMap::new();
        for (pc, profile) in &self.branch_profiles {
            result.insert(pc.to_string(), (profile.taken_count, profile.not_taken_count));
        }
        result
    }
    
    fn deserialize_branch_profiles(&mut self, data: HashMap<String, (u64, u64)>) {
        self.branch_profiles.clear();
        for (pc_str, (taken, not_taken)) in data {
            if let Ok(pc) = pc_str.parse::<usize>() {
                let mut profile = BranchProfile::new();
                profile.taken_count = taken;
                profile.not_taken_count = not_taken;
                self.branch_profiles.insert(pc, profile);
            }
        }
    }
}

impl Default for HotSpotProfiler {
    fn default() -> Self {
        Self::new()
    }
}

// Serialization support
#[derive(Serialize, Deserialize)]
struct ProfileData {
    function_counts: HashMap<usize, u64>,
    loop_counts: HashMap<usize, u64>,
    type_profiles: HashMap<String, HashMap<String, u64>>,
    branch_profiles: HashMap<String, (u64, u64)>,
}