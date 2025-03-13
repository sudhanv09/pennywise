using Microsoft.EntityFrameworkCore.Storage.ValueConversion;

namespace api.Data;

public class UlidConverter: ValueConverter<Ulid, string>
{
    private static readonly ConverterMappingHints DefaultHints = new (size: 26);

    public UlidConverter() : this(null)
    {
        
    }
    
    public UlidConverter(ConverterMappingHints? mappingHints = null): base(
        convertToProviderExpression: x => x.ToString(),
        convertFromProviderExpression: x => Ulid.Parse(x),
        mappingHints: DefaultHints.With(mappingHints))
    {
        
    }
}