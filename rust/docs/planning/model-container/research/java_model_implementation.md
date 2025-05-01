# Java Model Implementation Research

## Core Structure

The Java implementation of the Smithy Model is an immutable container with the following key components:

1. **Model Class** - The main container class that holds shapes and metadata
2. **Shape Storage** - A map of ShapeId to Shape instances
3. **Metadata Storage** - A map of metadata keys to Node values
4. **Caching Mechanisms** - Various caches for performance optimization
5. **Builder Pattern** - A builder for constructing models

## Key Features

### Immutability

The Model class is immutable, with all modifications done through the builder:
```java
public final class Model implements ToSmithyBuilder<Model> {
    // Immutable fields
    private final Map<String, Node> metadata;
    private final Map<ShapeId, Shape> shapeMap;
    
    // Builder pattern for modifications
    public static Builder builder() {
        return new Builder();
    }
    
    @Override
    public Builder toBuilder() {
        return builder()
                .metadata(getMetadata())
                .addShapes(this);
    }
}
```

### Shape Access

The Model provides various methods to access shapes:

1. **Direct access by ShapeId**:
```java
public Optional<Shape> getShape(ShapeId id) {
    return Optional.ofNullable(shapeMap.get(id));
}
```

2. **Access by shape type**:
```java
public <T extends Shape> Set<T> toSet(Class<T> shapeType) {
    return (Set<T>) cachedTypes.computeIfAbsent(shapeType, t -> {
        Set<T> result = new HashSet<>();
        for (Shape shape : shapeMap.values()) {
            if (shapeType.isInstance(shape)) {
                result.add((T) shape);
            }
        }
        return Collections.unmodifiableSet(result);
    });
}
```

3. **Streaming all shapes**:
```java
public Stream<Shape> shapes() {
    return shapeMap.values().stream();
}
```

4. **Streaming shapes of a specific type**:
```java
public <T extends Shape> Stream<T> shapes(Class<T> shapeType) {
    return toSet(shapeType).stream();
}
```

### Trait-Based Queries

The Model provides methods to find shapes with specific traits:

```java
public Set<Shape> getShapesWithTrait(ToShapeId trait) {
    Map<ShapeId, Set<Shape>> mappings = getTraitCache().traitIdsToShapes;
    return Collections.unmodifiableSet(mappings.getOrDefault(trait.toShapeId(), Collections.emptySet()));
}

public Set<Shape> getShapesWithTrait(Class<? extends Trait> trait) {
    Map<Class<? extends Trait>, Set<Shape>> mappings = getTraitCache().traitsToShapes;
    return Collections.unmodifiableSet(mappings.getOrDefault(trait, Collections.emptySet()));
}
```

### Caching

The implementation uses several caching mechanisms for performance:

1. **Type-based caching** - Caches shapes by their type:
```java
private final Map<Class<? extends Shape>, Set<? extends Shape>> cachedTypes = new ConcurrentHashMap<>();
```

2. **Trait-based caching** - Lazily computes and caches trait mappings:
```java
private volatile TraitCache traitCache;

private TraitCache getTraitCache() {
    TraitCache cache = traitCache;
    if (cache == null) {
        synchronized (this) {
            cache = traitCache;
            if (cache == null) {
                traitCache = cache = new TraitCache(this.shapeMap.values());
            }
        }
    }
    return cache;
}
```

3. **Knowledge Index** - Caches computed information about the model:
```java
private final Map<String, KnowledgeIndex> blackboard = new ConcurrentSkipListMap<>();

public <T extends KnowledgeIndex> T getKnowledge(Class<T> type, Function<Model, T> constructor) {
    return (T) blackboard.computeIfAbsent(type.getName(), t -> constructor.apply(this));
}
```

### Builder Pattern

The Model uses a builder pattern for construction:

```java
public static final class Builder implements SmithyBuilder<Model> {
    private final BuilderRef<Map<String, Node>> metadata = BuilderRef.forUnorderedMap();
    private final BuilderRef<Map<ShapeId, Shape>> shapeMap = BuilderRef.forUnorderedMap();
    
    // Methods to add/remove shapes and metadata
    public Builder addShape(Shape shape) {
        // Implementation
    }
    
    public Builder removeShape(ShapeId shapeId) {
        // Implementation
    }
    
    @Override
    public Model build() {
        return new Model(this);
    }
}
```

### Member Shape Handling

Member shapes are handled specially:

1. They're automatically added when their containing shape is added:
```java
public Builder addShape(Shape shape) {
    // Members must be added by their containing shapes.
    if (!shape.isMemberShape()) {
        shapeMap.get().put(shape.getId(), shape);
        // Automatically add members of the shape.
        for (MemberShape memberShape : shape.members()) {
            shapeMap.get().put(memberShape.getId(), memberShape);
        }
    }
    return this;
}
```

2. They're automatically removed when their containing shape is removed:
```java
public Builder removeShape(ShapeId shapeId) {
    if (shapeMap.hasValue() && shapeMap.peek().containsKey(shapeId)) {
        Shape previous = shapeMap.peek().get(shapeId);
        shapeMap.get().remove(shapeId);

        // Automatically remove any members contained in the shape.
        for (MemberShape memberShape : previous.members()) {
            shapeMap.get().remove(memberShape.getId());
        }
    }
    return this;
}
```

## Key Design Decisions

1. **Immutable Model** - The Model is immutable, with all modifications done through a builder
2. **Extensive Caching** - Various caching mechanisms for performance
3. **Type-Safe Access** - Methods to access shapes by type
4. **Trait-Based Queries** - Methods to find shapes with specific traits
5. **Knowledge Index** - Extensible system for computing and caching information about the model
6. **Special Member Handling** - Member shapes are automatically added/removed with their containing shapes
